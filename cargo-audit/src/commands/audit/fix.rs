//! The `cargo audit fix` subcommand

use crate::{auditor::Auditor, lockfile, prelude::*};
use abscissa_core::{Command, Runnable};
use cargo_lock::Lockfile;
use clap::Parser;
use rustsec::Fixer;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Command, Clone, Default, Debug, Parser)]
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
    /// Initialize `Auditor`
    pub fn auditor(&self) -> Auditor {
        Auditor::new(&APP.config())
    }

    /// Locate `Cargo.lock`
    pub fn cargo_lock_path(&self) -> Option<&Path> {
        self.file.as_deref()
    }
}

impl Runnable for FixCommand {
    fn run(&self) {
        let path = lockfile::locate_or_generate(self.cargo_lock_path()).unwrap_or_else(|e| {
            status_err!("{}", e);
            exit(2);
        });

        let report = self.auditor().audit_lockfile(&path);
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

        // This should always succeed because the auditor loaded it successfully already
        let lockfile = Lockfile::load(&path).expect("Failed to load Cargo.lock");

        // TODO: allow specifying mnanifest path
        let fixer = Fixer::new(None, lockfile);

        let dry_run = self.dry_run;
        let dry_run_info = if dry_run { " (dry run)" } else { "" };

        status_ok!(
            "Fixing",
            "vulnerable dependencies in `{}`{}",
            &path.display(),
            dry_run_info
        );

        for vulnerability in &report.vulnerabilities.list {
            if vulnerability.versions.patched().is_empty() {
                status_warn!(
                    "No patched versions available for {} in crate {}",
                    vulnerability.advisory.id,
                    vulnerability.package.name
                );
            } else {
                let mut command = fixer.get_fix_command(vulnerability, dry_run);
                // When calling `.status()` the stdout and stderr are inherited from the parent,
                // so any status or error messages from `cargo update` will automatically be forwarded
                // to the user of `cargo audit fix`.
                let status = command.status();
                match status {
                    Ok(_) => (),
                    Err(e) => status_warn!(
                        "Failed to run `cargo update` for package {}: {}",
                        vulnerability.package.name,
                        e
                    ),
                }
            }
        }

        // TODO: determine exit status depending on whether any vulnerabilities remain
    }
}
