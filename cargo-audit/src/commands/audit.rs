//! The `cargo audit` subcommand

use std::{fmt, path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand, ValueEnum};

use crate::{
    auditor::Auditor,
    config::{AuditConfig, DenyOption, OutputFormat},
    error::{Error, ErrorKind, display_err_with_source},
    lockfile,
    prelude::*,
    terminal::ColorChoice,
};

#[cfg(feature = "fix")]
mod fix;
#[cfg(feature = "fix")]
pub use fix::FixCommand;

#[cfg(feature = "binary-scanning")]
mod binary_scanning;
#[cfg(feature = "binary-scanning")]
pub use binary_scanning::BinCommand;

mod update_db;
pub use update_db::UpdateDbCommand;

/// The `cargo audit` subcommand
#[derive(Clone, Default, Debug, Parser)]
#[command(version)]
pub struct AuditCommand {
    /// Optional subcommand (used for `cargo audit fix` and `cargo audit bin`)
    #[command(subcommand)]
    pub(crate) subcommand: Option<AuditSubcommand>,

    /// Colored output configuration
    #[arg(
        short = 'c',
        long = "color",
        help = "color configuration (default: auto)"
    )]
    pub color: Option<Color>,

    /// Filesystem path to the advisory database git repository
    #[arg(
        short,
        long = "db",
        help = "advisory database git repo path (default: ~/.cargo/advisory-db)"
    )]
    pub(crate) db: Option<PathBuf>,

    /// Deny flag
    #[arg(
        short = 'D',
        long = "deny",
        help = "exit with an error on: warnings (any), unmaintained, unsound, yanked"
    )]
    pub(crate) deny: Vec<DenyOption>,

    /// Path to `Cargo.lock`
    #[arg(
        short = 'f',
        long = "file",
        help = "Cargo lockfile to inspect (or `-` for STDIN, default: Cargo.lock)"
    )]
    file: Option<PathBuf>,

    /// Advisory IDs to ignore
    #[arg(
        long = "ignore",
        value_name = "ADVISORY_ID",
        help = "Advisory id to ignore (can be specified multiple times)"
    )]
    pub(crate) ignore: Vec<String>,

    /// Warn about ignored advisories that no longer affect the lockfile
    #[arg(
        long,
        value_name = "BOOL",
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "true",
        help = "warn about unused ignore entries when auditing a lockfile (default: true)"
    )]
    pub(crate) warn_unused_ignores: Option<bool>,

    /// Skip checking for yanked crates
    #[arg(long = "no-yanked", help = "do not check for yanked crates")]
    pub(crate) no_yanked: bool,

    /// Skip fetching the advisory database git repository
    #[arg(
        short = 'n',
        long = "no-fetch",
        help = "do not perform a git fetch on the advisory DB"
    )]
    pub(crate) no_fetch: bool,

    /// Allow stale advisory databases that haven't been recently updated
    #[arg(long = "stale", help = "allow stale database")]
    pub(crate) stale: bool,

    /// Target CPU architecture to find vulnerabilities for
    #[arg(
        long = "target-arch",
        help = "filter vulnerabilities by CPU (default: no filter). Can be specified multiple times"
    )]
    pub(crate) target_arch: Vec<String>,

    /// Target OS to find vulnerabilities for
    #[arg(
        long = "target-os",
        help = "filter vulnerabilities by OS (default: no filter). Can be specified multiple times"
    )]
    pub(crate) target_os: Vec<String>,

    /// URL to the advisory database git repository
    #[arg(short = 'u', long = "url", help = "URL for advisory database git repo")]
    pub(crate) url: Option<String>,

    /// Quiet mode - avoids printing extraneous information
    #[arg(
        short = 'q',
        long = "quiet",
        help = "Avoid printing unnecessary information"
    )]
    pub(crate) quiet: bool,

    /// Output format
    #[arg(
        long = "format",
        value_name = "FORMAT",
        help = "Output format: terminal, json, or sarif"
    )]
    pub(crate) output_format: Option<OutputFormat>,

    /// Output reports as JSON
    #[arg(long = "json", help = "Output report in JSON format")]
    pub(crate) output_json: bool,
}

impl AuditCommand {
    /// Run the `cargo audit` subcommand with the given configuration
    pub fn run(self, config: AuditConfig) -> Result<ExitCode, Error> {
        let maybe_path = self.file.as_deref();
        // It is important to generate the lockfile before initializing the auditor,
        // otherwise we might deadlock because both need the Cargo package lock
        let path =
            lockfile::locate_or_generate(maybe_path).map_err(|e| ErrorKind::Parse.context(e))?;
        let mut auditor = Auditor::new(&config)?;
        let report = auditor.audit_lockfile(&path);
        Ok(match report {
            Ok(report) => match auditor.should_exit_with_failure(&report) {
                true => ExitCode::FAILURE,
                false => ExitCode::SUCCESS,
            },
            Err(e) => {
                status_err!("{}", display_err_with_source(&e));
                ExitCode::from(2)
            }
        })
    }
}

/// Subcommands of `cargo audit`
#[derive(Subcommand, Clone, Debug)]
pub enum AuditSubcommand {
    /// `cargo audit fix` subcommand
    #[cfg(feature = "fix")]
    #[command(about = "automatically upgrade vulnerable dependencies")]
    Fix(FixCommand),

    /// `cargo audit update-db` subcommand
    #[command(about = "update the advisory database")]
    UpdateDb(UpdateDbCommand),

    /// `cargo audit bin` subcommand
    #[cfg(feature = "binary-scanning")]
    #[command(
        about = "scan compiled binaries",
        long_about = "Scan compiled binaries for known vulnerabilities.

Performs a complete scan if the binary is built with 'cargo auditable'.
If not, recovers a part of the dependency list from panic messages."
    )]
    Bin(BinCommand),
}

/// Terminal color choices for output
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
#[value(rename_all = "kebab-case")] // If you change this, remember to update `fmt::Display` impl.
pub enum Color {
    /// Always use colored output
    Always,
    // TODO: Should this be also supported?
    // AlwaysAnsi,
    /// Automatically detect whether to use colored output
    #[default]
    Auto,
    /// Never use colored output
    Never,
}

impl From<Color> for ColorChoice {
    fn from(value: Color) -> Self {
        match value {
            Color::Always => Self::Always,
            Color::Auto => Self::Auto,
            Color::Never => Self::Never,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: This must be in sync with values genereted in ValueEnum implementation.
        match self {
            Self::Always => f.write_str("always"),
            Self::Auto => f.write_str("auto"),
            Self::Never => f.write_str("never"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_dependency_trees() {
        for show_tree in [true, false] {
            let mut config = AuditConfig::default();
            config.output.show_tree = show_tree;

            let command = AuditCommand::default();
            let mut overridden = config.clone();
            overridden.apply(&command).unwrap();
            assert_eq!(overridden.output.show_tree, show_tree);

            let command = AuditCommand {
                quiet: true,
                ..Default::default()
            };

            let mut overridden = config.clone();
            overridden.apply(&command).unwrap();
            assert!(!overridden.output.show_tree);
            assert!(overridden.output.quiet);
        }
    }

    /// Ensure that when the database fetch option is false in the config file
    /// that it takes precedence when the CLI --no-fetch flag is _not_ set.
    #[test]
    fn override_default_fetch_option() {
        // Assert the default value for the fetch option is true
        let mut config = AuditConfig::default();
        assert!(config.database.fetch);

        let mut audit_command = AuditCommand::default();
        let mut overridden_config = config.clone();
        overridden_config.apply(&audit_command).unwrap();
        assert!(overridden_config.database.fetch);

        // as the CLI flag --no-fetch is false when not provided
        // override_config should not change the fetch option
        // when it is set to false in the config file
        config.database.fetch = false;
        let mut overridden_config = config.clone();
        overridden_config.apply(&audit_command).unwrap();
        assert!(!overridden_config.database.fetch);

        config.database.fetch = true;
        audit_command.no_fetch = true;
        let mut overridden_config = config.clone();
        overridden_config.apply(&audit_command).unwrap();
        assert!(!overridden_config.database.fetch);
    }
}
