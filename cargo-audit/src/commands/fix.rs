//! The `cargo audit fix` subcommand

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::ExitCode,
};

use cargo_lock::Lockfile;
use clap::Parser;
use rustsec::{Fixer, advisory::Id};

use crate::{
    auditor::Auditor,
    config::AuditConfig,
    error::Error,
    lockfile,
    terminal::{status_err, status_ok, status_warn},
};

/// The `cargo audit fix` subcommand
#[derive(Clone, Default, Debug, Parser)]
#[command(author, version, about)]
pub struct FixCommand {
    /// Path to `Cargo.lock`
    #[arg(short = 'f', long = "file", help = "Cargo lockfile to inspect")]
    file: Option<PathBuf>,

    /// Perform a dry run
    #[arg(long = "dry-run", help = "perform a dry run for the fix")]
    dry_run: bool,
}

impl FixCommand {
    /// Locate `Cargo.lock`
    fn cargo_lock_path(&self) -> Option<&Path> {
        self.file.as_deref()
    }

    /// Run the `cargo audit fix` subcommand with the given configuration
    pub fn run(self, mut config: AuditConfig) -> Result<ExitCode, Error> {
        let path = match lockfile::locate_or_generate(self.cargo_lock_path()) {
            Ok(path) => path,
            Err(e) => {
                status_err!("{}", e);
                return Ok(ExitCode::from(2));
            }
        };

        let report = Auditor::new(&config)?.audit_lockfile(&path);
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

        let fixer = fixer_for(&path);

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

/// A [`Fixer`] for the lockfile at `path`, run with the `cargo` that invoked us if known
fn fixer_for(path: &Path) -> Fixer {
    // This should always succeed because the auditor loaded it successfully already
    let lockfile = Lockfile::load(path).expect("Failed to load Cargo.lock");

    // TODO: allow specifying manifest path
    let path_to_cargo: Option<PathBuf> = std::env::var_os("CARGO").map(|path| path.into());
    Fixer::new(lockfile, None, path_to_cargo)
}
