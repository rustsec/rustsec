//! The `cargo audit` subcommand

#[cfg(feature = "fix")]
mod fix;

use super::CargoAuditCommand;
use crate::{
    auditor::Auditor,
    config::{AuditConfig, DenyWarningOption, OutputFormat},
    prelude::*,
};
use abscissa_core::{config::Override, terminal::ColorChoice, FrameworkError};
use gumdrop::Options;
use rustsec::database::scope;
use rustsec::platforms::target::{Arch, OS};
use std::{path::PathBuf, process::exit};

#[cfg(feature = "fix")]
use self::fix::FixCommand;

/// The `cargo audit` subcommand
#[derive(Command, Default, Debug, Options)]
pub struct AuditCommand {
    /// Optional subcommand (used for `cargo audit fix`)
    #[cfg(feature = "fix")]
    #[options(command)]
    subcommand: Option<AuditSubcommand>,

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
        help = "exit with an error if any warning advisories of specified kinds are found"
    )]
    deny_warnings: Vec<DenyWarningOption>,

    /// Path to `Cargo.lock`
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

    /// Vulnerability querying does not consider local crates
    #[options(
        no_short,
        long = "no-local-crates",
        help = "Vulnerability querying does not consider local crates"
    )]
    no_local_crates: bool,
}

/// Subcommands of `cargo audit`
#[cfg(feature = "fix")]
#[derive(Command, Debug, Options, Runnable)]
pub enum AuditSubcommand {
    /// `cargo audit fix` subcommand
    #[options(help = "automatically upgrade vulnerable dependencies")]
    Fix(FixCommand),
}

impl AuditCommand {
    /// Get the color configuration
    pub fn color_config(&self) -> Option<ColorChoice> {
        self.color.as_ref().map(|colors| match colors.as_ref() {
            "always" => ColorChoice::Always,
            "never" => ColorChoice::Never,
            _ => panic!("invalid color choice setting: {}", &colors),
        })
    }
}

impl Override<AuditConfig> for AuditCommand {
    fn override_config(&self, mut config: AuditConfig) -> Result<AuditConfig, FrameworkError> {
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

        for kind in &self.deny_warnings {
            match kind {
                DenyWarningOption::All => {
                    config.output.deny_warnings = vec![
                        DenyWarningOption::Other,
                        DenyWarningOption::Unmaintained,
                        DenyWarningOption::Yanked,
                    ]
                }
                k => config.output.deny_warnings.push(*k),
            }
        }

        config.output.quiet |= self.quiet;

        if self.output_json {
            config.output.format = OutputFormat::Json;
        }

        if self.no_local_crates {
            config.packages.source = Some(scope::Registry::Public)
        }

        Ok(config)
    }
}

impl Runnable for AuditCommand {
    fn run(&self) {
        #[cfg(feature = "fix")]
        match &self.subcommand {
            Some(AuditSubcommand::Fix(fix)) => {
                fix.run();
                exit(0)
            }
            None => (),
        }

        if self.help {
            Self::print_usage_and_exit(&[]);
        }

        if self.version {
            println!("cargo-audit {}", CargoAuditCommand::version());
            exit(0);
        }

        let lockfile_path = self.file.as_deref();
        let report = self.auditor().audit(lockfile_path);

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
