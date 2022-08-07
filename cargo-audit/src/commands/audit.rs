//! The `cargo audit` subcommand

#[cfg(feature = "fix")]
mod fix;

use crate::{
    auditor::Auditor,
    config::{AuditConfig, DenyOption, OutputFormat},
    prelude::*,
};
use abscissa_core::{config::Override, terminal::ColorChoice, FrameworkError};
use clap::Parser;
use rustsec::platforms::target::{Arch, OS};
use std::{path::PathBuf, process::exit};

#[cfg(feature = "fix")]
use self::fix::FixCommand;
#[cfg(feature = "fix")]
use clap::Subcommand;

/// The `cargo audit` subcommand
#[derive(Command, Default, Debug, Parser)]
#[clap(version)]
pub struct AuditCommand {
    /// Optional subcommand (used for `cargo audit fix`)
    #[cfg(feature = "fix")]
    #[clap(subcommand)]
    subcommand: Option<AuditSubcommand>,

    /// Get help information
    #[clap(short = 'h', long = "help", help = "output help information and exit")]
    help: bool,

    /// Colored output configuration
    #[clap(
        short = 'c',
        long = "color",
        help = "color configuration: always, never (default: auto)"
    )]
    color: Option<String>,

    /// Filesystem path to the advisory database git repository
    #[clap(
        short,
        long = "db",
        help = "advisory database git repo path (default: ~/.cargo/advisory-db)"
    )]
    db: Option<PathBuf>,

    /// Deny flag
    #[clap(
        short = 'D',
        long = "deny",
        help = "exit with an error on: warnings (any), unmaintained, unsound, yanked"
    )]
    deny: Vec<DenyOption>,

    /// Path to `Cargo.lock`
    #[clap(
        short = 'f',
        long = "file",
        help = "Cargo lockfile to inspect (or `-` for STDIN, default: Cargo.lock)"
    )]
    file: Option<PathBuf>,

    /// Advisory IDs to ignore
    #[clap(
        long = "ignore",
        value_name = "ADVISORY_ID",
        help = "Advisory id to ignore (can be specified multiple times)"
    )]
    ignore: Vec<String>,

    /// Ignore the sources of packages in Cargo.toml
    #[clap(
        long = "ignore-source",
        help = "Ignore sources of packages in Cargo.toml, matching advisories regardless of source"
    )]
    ignore_source: bool,

    /// Skip fetching the advisory database git repository
    #[clap(
        short = 'n',
        long = "no-fetch",
        help = "do not perform a git fetch on the advisory DB"
    )]
    no_fetch: bool,

    /// Allow stale advisory databases that haven't been recently updated
    #[clap(long = "stale", help = "allow stale database")]
    stale: bool,

    /// Target CPU architecture to find vulnerabilities for
    #[clap(
        long = "target-arch",
        help = "filter vulnerabilities by CPU (default: no filter)"
    )]
    target_arch: Option<Arch>,

    /// Target OS to find vulnerabilities for
    #[clap(
        long = "target-os",
        help = "filter vulnerabilities by OS (default: no filter)"
    )]
    target_os: Option<OS>,

    /// URL to the advisory database git repository
    #[clap(short = 'u', long = "url", help = "URL for advisory database git repo")]
    url: Option<String>,

    /// Quiet mode - avoids printing extraneous information
    #[clap(
        short = 'q',
        long = "quiet",
        help = "Avoid printing unnecessary information"
    )]
    quiet: bool,

    /// Output reports as JSON
    #[clap(long = "json", help = "Output report in JSON format")]
    output_json: bool,
}

/// Subcommands of `cargo audit`
#[cfg(feature = "fix")]
#[derive(Subcommand, Debug, Runnable)]
pub enum AuditSubcommand {
    /// `cargo audit fix` subcommand
    #[clap(about = "automatically upgrade vulnerable dependencies")]
    Fix(FixCommand),
}

impl AuditCommand {
    /// Get the color configuration
    pub fn color_config(&self) -> Option<ColorChoice> {
        self.color.as_ref().map(|colors| match colors.as_ref() {
            "always" => ColorChoice::Always,
            "auto" => ColorChoice::Auto,
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

        config.advisories.ignore_source |= self.ignore_source;
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

        for kind in &self.deny {
            if *kind == DenyOption::Warnings {
                config.output.deny = DenyOption::all();
            } else {
                config.output.deny.push(*kind);
            }
        }

        config.output.quiet |= self.quiet;

        if self.output_json {
            config.output.format = OutputFormat::Json;
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

        let lockfile_path = self.file.as_deref();
        let report = self.auditor().audit(lockfile_path);

        match report {
            Ok(report) => {
                if report.vulnerabilities.found {
                    exit(1);
                }
                exit(0);
            }
            Err(e) => {
                status_err!("{}", e);
                exit(2);
            }
        };
    }
}

impl AuditCommand {
    /// Initialize `Auditor`
    pub fn auditor(&self) -> Auditor {
        Auditor::new(&APP.config())
    }
}
