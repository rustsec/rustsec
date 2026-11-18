//! The `cargo audit fix` subcommand

mod via_dependents;

use crate::{auditor::Auditor, config::AuditConfig, lockfile, prelude::*};
use cargo_lock::Lockfile;
use clap::Parser;
use rustsec::{Fixer, Report, Vulnerability, advisory::Id};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, exit},
};

#[derive(Clone, Default, Debug, Parser)]
#[command(author, version, about)]
pub(crate) struct FixCommand {
    /// Path to `Cargo.lock`
    #[arg(short = 'f', long = "file", help = "Cargo lockfile to inspect")]
    file: Option<PathBuf>,

    /// Perform a dry run
    #[arg(long = "dry-run", help = "perform a dry run for the fix")]
    dry_run: bool,

    /// Also update dependents that pin a vulnerable crate below its fix
    #[arg(
        long = "via-dependents",
        help = "when a dependent pins a vulnerable crate below its fix, update that dependent"
    )]
    via_dependents: bool,
}

impl FixCommand {
    /// Locate `Cargo.lock`
    fn cargo_lock_path(&self) -> Option<&Path> {
        self.file.as_deref()
    }

    /// Run the `cargo audit fix` subcommand with the given configuration
    pub(crate) fn run(&self, config: &AuditConfig) {
        let path = lockfile::locate_or_generate(self.cargo_lock_path()).unwrap_or_else(|e| {
            status_err!("{}", e);
            exit(2);
        });

        let report = Auditor::new(config).audit_lockfile(&path);
        let report = match report {
            Ok(report) => {
                // TODO: also handle warnings
                if report.vulnerabilities.list.is_empty() {
                    exit(0);
                }
                report
            }
            Err(e) => {
                status_err!("{}", e);
                exit(2);
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
                let command = fixer.get_fix_command(vulnerability, dry_run);
                if self
                    .run_cargo_update(command, vulnerability.package.name.as_str())
                    .is_none()
                {
                    failed_patches += 1;
                }
            }
        }

        if dry_run && self.via_dependents {
            failed_patches +=
                self.update_pinning_dependents(config, &path, &report.vulnerabilities.list, true);
        }
        if failed_patches != 0 {
            exit(2);
        }
        if dry_run {
            // When performing a dry run, the exit status is determined by whether we had any issues along the way
            if !unpatchable_vulns.is_empty() {
                exit(1);
            } else {
                exit(0)
            }
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
            let report_after_fix = self.audit_after_fix(&config, &path);
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
        }
    }

    /// Run a `cargo update` command for `package`, in the lockfile's directory
    /// when `--file` was given. Returns `None` if the command could not be started.
    fn run_cargo_update(&self, mut command: Command, package: &str) -> Option<ExitStatus> {
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
        command
            .status()
            .inspect_err(|e| {
                status_warn!(
                    "Failed to run `cargo update` for package {}: {}",
                    package,
                    e
                );
            })
            .ok()
    }

    /// Run a planned `cargo update --precise` for `package`. Unlike the plain
    /// update, cargo rejecting it counts as a failure: the plan may rest on a
    /// guessed `Cargo.toml` requirement that only cargo can check.
    fn run_precise_update(&self, command: Command, package: &str) -> bool {
        let Some(status) = self.run_cargo_update(command, package) else {
            return false;
        };
        if !status.success() {
            status_warn!("`cargo update` rejected the planned update of {}", package);
        }
        status.success()
    }

    /// Re-audit the lockfile after the plain updates, first updating pinning
    /// dependents when `--via-dependents` was given.
    ///
    /// Each [`Auditor`] is dropped right after its audit because it holds the
    /// Cargo package lock, which the `cargo update` commands need.
    fn audit_after_fix(&self, config: &AuditConfig, path: &Path) -> Report {
        let report = Auditor::new(config).audit_lockfile(path).unwrap();
        if !self.via_dependents {
            return report;
        }
        if self.update_pinning_dependents(config, path, &report.vulnerabilities.list, false) != 0 {
            exit(2);
        }
        Auditor::new(config).audit_lockfile(path).unwrap()
    }

    /// Plan and run `cargo update --precise` for dependents that pin any of
    /// `vulnerabilities` below its fix. Returns the number of commands that
    /// could not be started or that cargo rejected.
    fn update_pinning_dependents(
        &self,
        config: &AuditConfig,
        path: &Path,
        vulnerabilities: &[Vulnerability],
        dry_run: bool,
    ) -> usize {
        let fixer = fixer_for(path);
        via_dependents::plan_dependent_updates(config, &fixer, vulnerabilities, dry_run)
            .into_iter()
            .map(|(package, command)| self.run_precise_update(command, &package))
            .filter(|succeeded| !succeeded)
            .count()
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
