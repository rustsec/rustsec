//! The `cargo audit fix` subcommand

use crate::{auditor::Auditor, config::AuditConfig, lockfile, prelude::*};
use cargo_lock::Lockfile;
use clap::Parser;
use rustsec::{
    Fixer, Report,
    advisory::{Id, Metadata, Versions},
    package::Package,
};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::exit,
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
            Ok(report) => report,
            Err(e) => {
                status_err!("{}", e);
                exit(2);
            }
        };

        let advisories = AdvisoryMetadata::extract(&report).collect::<Vec<_>>();
        if advisories.is_empty() {
            exit(0);
        }

        // This should always succeed because the auditor loaded it successfully already
        let lockfile = Lockfile::load(&path).expect("Failed to load Cargo.lock");

        // TODO: allow specifying manifest path
        let path_to_cargo: Option<PathBuf> = std::env::var_os("CARGO").map(|path| path.into());
        let fixer = Fixer::new(lockfile, None, path_to_cargo);

        let dry_run = self.dry_run;
        if dry_run {
            status_warn!("Performing a dry run, the fixes will not be applied");
        }

        let mut unpatchable_advisories: BTreeSet<Id> = BTreeSet::new();
        let mut failed_patches = 0;

        for advisory in &advisories {
            if advisory.versions.patched().is_empty() {
                unpatchable_advisories.insert(advisory.advisory.id.clone());
                status_warn!(
                    "No patched versions available for {} in crate {}",
                    advisory.advisory.id,
                    advisory.package.name
                );
            } else {
                let mut command =
                    fixer.fix_command_for(advisory.package, advisory.versions, dry_run);
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
                        advisory.package.name,
                        e
                    );
                }
            }
        }

        if failed_patches != 0 {
            exit(2);
        }
        if dry_run {
            // When performing a dry run, the exit status is determined by whether we had any issues along the way
            if !unpatchable_advisories.is_empty() {
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
                "that the advisories are fixed after updating dependencies"
            );
            let mut config = config.clone();
            config.output.quiet = true;
            let mut auditor = Auditor::new(&config);

            let report_after_fix = auditor.audit_lockfile(&path).unwrap();
            let advisories_after_fix =
                AdvisoryMetadata::extract(&report_after_fix).collect::<Vec<_>>();
            let fixable_but_unfixed: Vec<String> = advisories_after_fix
                .iter()
                .filter(|advisory| !unpatchable_advisories.contains(&advisory.advisory.id))
                .map(|advisory| advisory.advisory.id.to_string())
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

            let remaining_advisories_count = advisories_after_fix.len();
            let fixed_advisories_count =
                advisories.len().saturating_sub(remaining_advisories_count);
            if fixed_advisories_count != 0 {
                if remaining_advisories_count == 0 {
                    status_ok!("Fixed", "{} advisories", fixed_advisories_count);
                } else {
                    status_warn!(
                        "Fixed {} advisories but {} remain",
                        fixed_advisories_count,
                        remaining_advisories_count
                    );
                }
            }
        }
    }
}

/// Advisory data that `cargo audit fix` acts on, borrowed from a [`Report`].
struct AdvisoryMetadata<'a> {
    advisory: &'a Metadata,
    versions: &'a Versions,
    package: &'a Package,
}

impl<'a> AdvisoryMetadata<'a> {
    /// Extract the advisories to fix: all vulnerabilities, plus warnings that
    /// carry advisory and affected-version information.
    ///
    /// Yanked packages do not have this data and need a different fix strategy.
    /// The initial report and the re-audit after the update both use this, so
    /// the fixed and remaining counts compare the same kind of items.
    fn extract(report: &'a Report) -> impl Iterator<Item = AdvisoryMetadata<'a>> {
        report
            .vulnerabilities
            .list
            .iter()
            .map(|vulnerability| Self {
                advisory: &vulnerability.advisory,
                versions: &vulnerability.versions,
                package: &vulnerability.package,
            })
            .chain(report.warnings.values().flatten().filter_map(|warning| {
                Some(Self {
                    advisory: warning.advisory.as_ref()?,
                    versions: warning.versions.as_ref()?,
                    package: &warning.package,
                })
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustsec::{Database, Lockfile, Warning, WarningKind, report::Settings};

    #[test]
    fn warnings_without_advisory_or_versions_are_not_fix_targets() {
        let lockfile: Lockfile =
            "version = 3\n[[package]]\nname = \"example\"\nversion = \"0.1.0\"\n"
                .parse()
                .unwrap();
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir(dir.path().join("crates")).unwrap();
        let database = Database::open(dir.path()).unwrap();
        let mut report = Report::generate(&database, &lockfile, &Settings::default());
        report
            .warnings
            .entry(WarningKind::Yanked)
            .or_default()
            .push(Warning::new(
                WarningKind::Yanked,
                &lockfile.packages[0],
                None,
                None,
                None,
            ));
        assert!(AdvisoryMetadata::extract(&report).next().is_none());
    }
}
