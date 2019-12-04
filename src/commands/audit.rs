//! The `cargo audit` subcommand

use super::CargoAuditCommand;
use crate::{
    auditor::Auditor,
    config::{AuditConfig, OutputFormat},
    prelude::*,
};
use abscissa_core::{config::Override, FrameworkError};
#[cfg(feature = "fix")]
use cargo_edit::{Dependency as EditDependency, LocalManifest};
use gumdrop::Options;
use rustsec::platforms::target::{Arch, OS};
#[cfg(feature = "fix")]
use rustsec::Vulnerability;
use std::{path::PathBuf, process::exit};

/// The `cargo audit` subcommand
#[derive(Command, Default, Debug, Options)]
pub struct AuditCommand {
    /// Get help information
    #[options(short = "h", long = "help", help = "output help information and exit")]
    help: bool,

    /// Get version information
    #[options(no_short, long = "version", help = "output version and exit")]
    version: bool,

    /// Colored output configuration
    #[options(
        short = "c",
        long = "color",
        help = "color configuration: always, never (default: auto)"
    )]
    color: Option<String>,

    /// Filesystem path to the advisory database git repository
    #[options(
        long = "db",
        help = "advisory database git repo path (default: ~/.cargo/advisory-db)"
    )]
    db: Option<PathBuf>,

    /// Deny warnings
    #[options(
        short = "D",
        long = "deny-warnings",
        help = "exit with an error if any warning advisories are found"
    )]
    deny_warnings: bool,

    /// Path to `Cargo.lock`
    #[options(
        short = "f",
        long = "file",
        help = "Cargo lockfile to inspect (or `-` for STDIN, default: Cargo.lock)"
    )]
    file: Option<PathBuf>,

    /// Attempt to update vulnerable dependencies
    #[cfg(feature = "fix")]
    #[options(no_short, long = "fix", help = "upgrade vulnerable dependencies")]
    fix: bool,

    /// Advisory IDs to ignore
    #[options(
        no_short,
        long = "ignore",
        meta = "ADVISORY_ID",
        help = "Advisory id to ignore (can be specified multiple times)"
    )]
    ignore: Vec<String>,

    /// Skip fetching the advisory database git repository
    #[options(
        short = "n",
        long = "no-fetch",
        help = "do not perform a git fetch on the advisory DB"
    )]
    no_fetch: bool,

    /// Allow stale advisory databases that haven't been recently updated
    #[options(no_short, long = "stale", help = "allow stale database")]
    stale: bool,

    /// Target CPU architecture to find vulnerabilities for
    #[options(
        no_short,
        long = "target-arch",
        help = "filter vulnerabilities by CPU (default: no filter)"
    )]
    target_arch: Option<Arch>,

    /// Target OS to find vulnerabilities for
    #[options(
        no_short,
        long = "target-os",
        help = "filter vulnerabilities by OS (default: no filter)"
    )]
    target_os: Option<OS>,

    /// URL to the advisory database git repository
    #[options(short = "u", long = "url", help = "URL for advisory database git repo")]
    url: Option<String>,

    /// Quiet mode - avoids printing extraneous information
    #[options(
        short = "q",
        long = "quiet",
        help = "Avoid printing unnecessary information"
    )]
    quiet: bool,

    /// Output reports as JSON
    #[options(no_short, long = "json", help = "Output report in JSON format")]
    output_json: bool,
}

impl Override<AuditConfig> for AuditCommand {
    fn override_config(&self, mut config: AuditConfig) -> Result<AuditConfig, FrameworkError> {
        if let Some(color) = &self.color {
            config.output.color = Some(color.clone());
        }

        if let Some(db) = &self.db {
            config.database.path = Some(db.into());
        }

        for advisory_id in &self.ignore {
            config
                .advisories
                .ignore
                .push(advisory_id.parse().unwrap_or_else(|e| {
                    status_err!("error parsing {}: {}", advisory_id, e);
                    exit(1);
                }));
        }

        config.database.fetch |= !self.no_fetch;
        config.database.stale |= self.stale;

        if let Some(target_arch) = self.target_arch {
            config.target.arch = Some(target_arch);
        }

        if let Some(target_os) = self.target_os {
            config.target.os = Some(target_os);
        }

        if let Some(url) = &self.url {
            config.database.url = Some(url.clone())
        }

        config.output.deny_warnings |= self.deny_warnings;
        config.output.quiet |= self.quiet;

        if self.output_json {
            config.output.format = OutputFormat::Json;
        }

        Ok(config)
    }
}

impl Runnable for AuditCommand {
    fn run(&self) {
        if self.help {
            Self::print_usage_and_exit(&[]);
        }

        if self.version {
            println!("cargo-audit {}", CargoAuditCommand::version());
            exit(0);
        }

        let lockfile_path = self.file.as_ref().map(PathBuf::as_path);
        let report = self.auditor().audit(lockfile_path);

        #[cfg(feature = "fix")]
        self.perform_fix(&report.vulnerabilities.list);

        if report.vulnerabilities.found {
            exit(1)
        }
    }
}

impl AuditCommand {
    /// Initialize `Auditor`
    pub fn auditor(&self) -> Auditor {
        let config = app_config();
        Auditor::new(&config)
    }

    /// Attempt to upgrade vulnerable dependencies
    #[cfg(feature = "fix")]
    pub fn perform_fix(&self, vulnerabilities: &[Vulnerability]) {
        if !self.fix {
            return;
        }

        let cargo_toml = self.cargo_toml_path();

        let mut manifest = LocalManifest::try_new(&cargo_toml).unwrap_or_else(|e| {
            status_err!(
                "couldn't load manifest from {}: {}",
                cargo_toml.display(),
                e
            );
            exit(1);
        });

        for vulnerability in vulnerabilities {
            if let Some(version) = vulnerability.versions.patched.get(0) {
                manifest
                    .upgrade(
                        &EditDependency::new(vulnerability.package.name.as_str())
                            .set_version(&version.to_string()),
                        false,
                    )
                    .unwrap_or_else(|e| status_warn!("unable to perform upgrade: {}", e));
            } else {
                status_warn!("no upgrade available for {}", vulnerability.package.name);
            }
        }
    }

    /// Locate `Cargo.toml`
    // TODO(tarcieri): less contrived implementation
    pub fn cargo_toml_path(&self) -> PathBuf {
        PathBuf::from("Cargo.toml")
    }
}
