//! The `cargo audit` subcommand

#[cfg(feature = "fix")]
use std::{collections::BTreeSet, path::Path};
use std::{
    fmt,
    io::{self, IsTerminal},
    path::PathBuf,
    process::ExitCode,
};

#[cfg(feature = "fix")]
use cargo_lock::Lockfile;
use clap::{Parser, Subcommand, ValueEnum};
#[cfg(feature = "fix")]
use rustsec::{Fixer, advisory::Id};

use crate::{
    auditor::{Auditor, update_db},
    config::{AuditConfig, DenyOption, FilterList, OutputFormat},
    error::{Error, ErrorKind, display_err_with_source},
    lockfile,
    prelude::*,
    terminal::ColorChoice,
};

/// The `cargo audit` subcommand
#[derive(Clone, Default, Debug, Parser)]
#[command(version)]
pub struct AuditCommand {
    /// Optional subcommand (used for `cargo audit fix` and `cargo audit bin`)
    #[command(subcommand)]
    subcommand: Option<AuditSubcommand>,

    /// Colored output configuration
    #[arg(
        short = 'c',
        long = "color",
        help = "color configuration (default: auto)"
    )]
    color: Option<Color>,

    /// Filesystem path to the advisory database git repository
    #[arg(
        short,
        long = "db",
        help = "advisory database git repo path (default: ~/.cargo/advisory-db)"
    )]
    db: Option<PathBuf>,

    /// Deny flag
    #[arg(
        short = 'D',
        long = "deny",
        help = "exit with an error on: warnings (any), unmaintained, unsound, yanked"
    )]
    deny: Vec<DenyOption>,

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
    ignore: Vec<String>,

    /// Skip checking for yanked crates
    #[arg(long = "no-yanked", help = "do not check for yanked crates")]
    no_yanked: bool,

    /// Skip fetching the advisory database git repository
    #[arg(
        short = 'n',
        long = "no-fetch",
        help = "do not perform a git fetch on the advisory DB"
    )]
    no_fetch: bool,

    /// Allow stale advisory databases that haven't been recently updated
    #[arg(long = "stale", help = "allow stale database")]
    stale: bool,

    /// Target CPU architecture to find vulnerabilities for
    #[arg(
        long = "target-arch",
        help = "filter vulnerabilities by CPU (default: no filter). Can be specified multiple times"
    )]
    target_arch: Vec<String>,

    /// Target OS to find vulnerabilities for
    #[arg(
        long = "target-os",
        help = "filter vulnerabilities by OS (default: no filter). Can be specified multiple times"
    )]
    target_os: Vec<String>,

    /// URL to the advisory database git repository
    #[arg(short = 'u', long = "url", help = "URL for advisory database git repo")]
    url: Option<String>,

    /// Quiet mode - avoids printing extraneous information
    #[arg(
        short = 'q',
        long = "quiet",
        help = "Avoid printing unnecessary information"
    )]
    quiet: bool,

    /// Output format
    #[arg(
        long = "format",
        value_name = "FORMAT",
        help = "Output format: terminal, json, or sarif"
    )]
    output_format: Option<OutputFormat>,

    /// Output reports as JSON
    #[arg(long = "json", help = "Output report in JSON format")]
    output_json: bool,
}

impl AuditCommand {
    /// Get the color configuration
    pub(crate) fn term_colors(&self) -> ColorChoice {
        if let Some(color) = self.color {
            return color.into();
        }

        match std::env::var("CARGO_TERM_COLOR") {
            Ok(e) if e == "always" => ColorChoice::Always,
            Ok(e) if e == "never" => ColorChoice::Never,
            Ok(e) if e == "auto" => ColorChoice::Auto,
            _ => match io::stdout().is_terminal() {
                true => ColorChoice::Auto,
                false => ColorChoice::Never,
            },
        }
    }

    /// Override loaded config with explicit command-line arguments
    pub fn override_config(&self, config: AuditConfig) -> Result<AuditConfig, Error> {
        let mut config = config;
        if let Some(db) = &self.db {
            config.database.path = Some(db.into());
        }

        for advisory_id in &self.ignore {
            config.advisories.ignore.push(
                advisory_id
                    .parse()
                    .map_err(|e| ErrorKind::Parse.context(e))?,
            );
        }

        config.database.fetch &= !self.no_fetch;
        config.database.stale |= self.stale;

        if !self.target_arch.is_empty() {
            config.target.arch = Some(FilterList::Many(self.target_arch.clone()));
        }

        if !self.target_os.is_empty() {
            config.target.os = Some(FilterList::Many(self.target_os.clone()));
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
        if self.quiet {
            config.output.show_tree = false;
        }

        // Handle output format (--json flag takes precedence for backward compatibility)
        if self.output_json {
            config.output.format = OutputFormat::Json;
        } else if let Some(format) = self.output_format {
            config.output.format = format;
        }

        if self.no_yanked {
            config.yanked.enabled = false;
        }

        Ok(config)
    }

    /// Run the `cargo audit` subcommand with the given configuration
    pub fn run(&self, config: &AuditConfig) -> Result<ExitCode, Error> {
        match &self.subcommand {
            #[cfg(feature = "binary-scanning")]
            Some(AuditSubcommand::Bin(bin)) => {
                return bin.run(config);
            }
            #[cfg(feature = "fix")]
            Some(AuditSubcommand::Fix(fix)) => {
                return fix.run(config);
            }
            Some(AuditSubcommand::UpdateDb(update_db)) => {
                return update_db.run(config);
            }
            None => {}
        }

        let maybe_path = self.file.as_deref();
        // It is important to generate the lockfile before initializing the auditor,
        // otherwise we might deadlock because both need the Cargo package lock
        let path =
            lockfile::locate_or_generate(maybe_path).map_err(|e| ErrorKind::Parse.context(e))?;
        let mut auditor = Auditor::new(config)?;
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
pub(super) enum AuditSubcommand {
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

#[cfg(feature = "fix")]
#[derive(Clone, Default, Debug, Parser)]
#[command(author, version, about)]
pub(crate) struct FixCommand {
    /// Path to `Cargo.lock`
    #[arg(short = 'f', long = "file", help = "Cargo lockfile to inspect")]
    file: Option<PathBuf>,

    /// Perform a dry run
    #[arg(long = "dry-run", help = "perform a dry run for the fix")]
    dry_run: bool,
}

#[cfg(feature = "fix")]
impl FixCommand {
    /// Locate `Cargo.lock`
    fn cargo_lock_path(&self) -> Option<&Path> {
        self.file.as_deref()
    }

    /// Run the `cargo audit fix` subcommand with the given configuration
    pub(crate) fn run(&self, config: &AuditConfig) -> Result<ExitCode, Error> {
        let path = match lockfile::locate_or_generate(self.cargo_lock_path()) {
            Ok(path) => path,
            Err(e) => {
                status_err!("{}", e);
                return Ok(ExitCode::from(2));
            }
        };

        let report = Auditor::new(config)?.audit_lockfile(&path);
        let report = match report {
            Ok(report) => {
                // TODO: also handle warnings
                if report.vulnerabilities.list.is_empty() {
                    return Ok(ExitCode::SUCCESS);
                }
                report
            }
            Err(e) => {
                status_err!("{}", e);
                return Ok(ExitCode::from(2));
            }
        };

        // This should always succeed because the auditor loaded it successfully already
        let lockfile = Lockfile::load(&path).expect("Failed to load Cargo.lock");

        // TODO: allow specifying manifest path
        let path_to_cargo: Option<PathBuf> = std::env::var_os("CARGO").map(|path| path.into());
        let fixer = Fixer::new(lockfile, None, path_to_cargo);

        let dry_run = self.dry_run;
        if dry_run {
            status_warn!("Performing a dry run, the fixes will not be applied");
        }

        let mut unpatchable_vulns: BTreeSet<Id> = BTreeSet::new();
        let mut failed_patches = 0;

        for vulnerability in &report.vulnerabilities.list {
            if vulnerability.versions.patched().is_empty() {
                unpatchable_vulns.insert(vulnerability.advisory.id.clone());
                status_warn!(
                    "No patched versions available for {} in crate {}",
                    vulnerability.advisory.id,
                    vulnerability.package.name
                );
            } else {
                let mut command = fixer.get_fix_command(vulnerability, dry_run);
                // If the path to Cargo.lock has been specified explicitly,
                // run the `cargo update` command in that directory
                if let Some(path) = self.cargo_lock_path() {
                    // documentation on .current_dir() recommends canonicalizing the path
                    let canonical_path = path.canonicalize().unwrap();
                    let dir = canonical_path.parent().unwrap();
                    command.current_dir(dir);
                }
                // When calling `.status()` the stdout and stderr are inherited from the parent,
                // so any status or error messages from `cargo update` will automatically be forwarded
                // to the user of `cargo audit fix`.
                let status = command.status();
                if let Err(e) = status {
                    failed_patches += 1;
                    status_warn!(
                        "Failed to run `cargo update` for package {}: {}",
                        vulnerability.package.name,
                        e
                    );
                }
            }
        }

        if failed_patches != 0 {
            return Ok(ExitCode::from(2));
        }
        if dry_run {
            // When performing a dry run, the exit status is determined by whether we had any issues along the way
            Ok(if !unpatchable_vulns.is_empty() {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        } else {
            // It is possible that some vulns we tried to fix actually weren't fixed,
            // either because there is no semver-compatible fix or because Cargo.toml version specification
            // is too restrictive (uses e.g. `=` or `=<` operators).
            status_ok!(
                "Verifying",
                "that the vulnerabilities are fixed after updating dependencies"
            );
            let mut config = config.clone();
            config.output.quiet = true;
            let mut auditor = Auditor::new(&config)?;

            let report_after_fix = auditor.audit_lockfile(&path)?;
            let vulns_after_fix = &report_after_fix.vulnerabilities.list;
            let fixable_but_unfixed: Vec<String> = vulns_after_fix
                .iter()
                .filter(|vuln| !unpatchable_vulns.contains(&vuln.advisory.id))
                .map(|vuln| vuln.advisory.id.to_string())
                .collect();
            if !fixable_but_unfixed.is_empty() {
                status_warn!(
                    "The following advisories have patched versions but could not be fixed:\n    {}\n\
                    This usually occurs when the fixed version is not semver-compatible,\n\
                    or the version range specified in your `Cargo.toml` is too restrictive\n\
                    (e.g. uses `=` or `=<` operators) so the fixed version would not match it.",
                    fixable_but_unfixed.join(", ")
                );
            }

            let remaining_vulns_count = report_after_fix.vulnerabilities.list.len();
            let fixed_vulns_count = report
                .vulnerabilities
                .list
                .len()
                .saturating_sub(remaining_vulns_count);
            if fixed_vulns_count != 0 {
                if remaining_vulns_count == 0 {
                    status_ok!("Fixed", "{} vulnerabilities", fixed_vulns_count);
                } else {
                    status_warn!(
                        "Fixed {} vulnerabilities but {} remain",
                        fixed_vulns_count,
                        remaining_vulns_count
                    );
                }
            }

            Ok(match remaining_vulns_count {
                0 => ExitCode::SUCCESS,
                _ => ExitCode::FAILURE,
            })
        }
    }
}

#[cfg(feature = "binary-scanning")]
/// The `cargo audit` subcommand
#[derive(Clone, Default, Debug, Parser)]
#[command()]
pub(crate) struct BinCommand {
    /// Maximum binary size in bytes to read
    #[arg(
        long = "max-binary-size",
        value_name = "BYTES",
        help = "Maximum binary size in bytes to read (default: 100MB; use 0 for unlimited)"
    )]
    max_binary_size: Option<u64>,

    /// Maximum audit data size in bytes to parse
    #[arg(
        long = "audit-data-size-limit",
        value_name = "BYTES",
        help = "Maximum audit data size in bytes to parse (default: 8MB)"
    )]
    audit_data_size_limit: Option<usize>,

    /// Paths to the binaries to be scanned
    #[arg(
        value_parser,
        required = true,
        help = "Paths to the binaries to be scanned"
    )]
    binary_paths: Vec<PathBuf>,
}

#[cfg(feature = "binary-scanning")]
impl BinCommand {
    /// Run the `cargo audit bin` subcommand with the given configuration
    pub(crate) fn run(&self, config: &AuditConfig) -> Result<ExitCode, Error> {
        let mut auditor = Auditor::new(config)?;
        auditor.set_binary_scan_limits(self.max_binary_size, self.audit_data_size_limit);
        let report = auditor.audit_binaries(&self.binary_paths);
        Ok(if report.vulnerabilities_found {
            ExitCode::from(1)
        } else if report.errors_encountered {
            ExitCode::from(2)
        } else {
            ExitCode::from(0)
        })
    }
}

#[derive(Clone, Default, Debug, Parser)]
#[command(author, version, about)]
pub(crate) struct UpdateDbCommand {
    /// Filesystem path to the advisory database git repository
    #[arg(
        short,
        long = "db",
        help = "advisory database git repo path (default: ~/.cargo/advisory-db)"
    )]
    db: Option<PathBuf>,
    /// URL to the advisory database git repository
    #[arg(short = 'u', long = "url", help = "URL for advisory database git repo")]
    url: Option<String>,
}

impl UpdateDbCommand {
    pub(crate) fn run(&self, config: &AuditConfig) -> Result<ExitCode, Error> {
        let mut config = config.clone();
        if let Some(db) = &self.db {
            config.database.path = Some(db.clone());
        }

        if let Some(url) = &self.url {
            config.database.url = Some(url.clone());
        }

        let path = config.database.path();
        let _ = update_db(&path, &config.database, config.output.is_quiet())?;
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
#[value(rename_all = "kebab-case")] // If you change this, remember to update `fmt::Display` impl.
enum Color {
    Always,
    // TODO: Should this be also supported?
    // AlwaysAnsi,
    #[default]
    Auto,
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

    /// Ensure that when the database fetch option is false in the config file
    /// that it takes precedence when the CLI --no-fetch flag is _not_ set.
    #[test]
    fn override_default_fetch_option() {
        // Assert the default value for the fetch option is true
        let mut config: AuditConfig = AuditConfig::default();
        assert!(config.database.fetch);

        let mut audit_command = AuditCommand::default();

        let overridden_config = audit_command.override_config(config.clone()).unwrap();
        assert!(overridden_config.database.fetch);

        // as the CLI flag --no-fetch is false when not provided
        // override_config should not change the fetch option
        // when it is set to false in the config file
        config.database.fetch = false;
        let overridden_config = audit_command.override_config(config.clone()).unwrap();
        assert!(!overridden_config.database.fetch);

        config.database.fetch = true;
        audit_command.no_fetch = true;
        let overridden_config = audit_command.override_config(config.clone()).unwrap();
        assert!(!overridden_config.database.fetch);
    }
}
