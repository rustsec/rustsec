//! The `cargo audit` subcommand

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use abscissa_core::{config::Override, FrameworkError};
use cargo_edit::Dependency;
use cargo_edit::LocalManifest;
use gumdrop::Options;
use rustsec::platforms::target::{Arch, OS};

use crate::{
    auditor::Auditor,
    config::{AuditConfig, OutputFormat},
    prelude::*,
};

use super::CargoAuditCommand;

/// The `cargo audit` subcommand
#[derive(Command, Default, Debug, Options)]
pub struct AuditCommand {
    /// Get help information
    #[options(short = "h", long = "help", help = "output help information and exit")]
    help: bool,

    /// Get version information
    #[options(no_short, long = "version", help = "output version and exit")]
    version: bool,

    /// Get version information
    /// TODO :: If the upgrade is a breaking change according to semver,
    ///         then we must inform the user of this.
    #[options(no_short, long = "fix", help = "upgrade unsafe cargo dependencies")]
    perform_fix: bool,

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

    /// Path to the lockfile
    #[options(
        short = "f",
        long = "file",
        help = "Cargo lockfile to inspect (or `-` for STDIN, default: Cargo.lock)"
    )]
    file: Option<PathBuf>,

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

        let report = self.auditor().audit(&lockfile_path);
        let path = Path::new("Cargo.toml");
        let mut manifest = match LocalManifest::try_new(&path) {
            Ok(ok) => ok,
            Err(err) => panic!("error:{:#?}", err),
        };
        if self.perform_fix {
            report
                .vulnerabilities
                .list
                .iter()
                .for_each(|vulnerability| {
                    if vulnerability.versions.patched.is_empty() {
                        println!("no upgrade available for {}", vulnerability.package.name);
                    } else {
                        manifest
                            .upgrade(
                                &Dependency::new(vulnerability.package.name.as_str()).set_version(
                                    vulnerability.versions.patched[0].to_string().as_str(), // this does not look good at all...
                                ),
                                false,
                            )
                            .expect("unable to perform upgrade.");
                    }
                });
        }
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
}
