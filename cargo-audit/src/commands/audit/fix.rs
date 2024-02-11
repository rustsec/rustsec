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

    /// Locate `Cargo.toml`
    // TODO(tarcieri): ability to specify path
    pub fn cargo_toml_path(&self) -> PathBuf {
        PathBuf::from("Cargo.toml")
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
        let lockfile =
            Lockfile::load("tests/examples/Cargo.lock").expect("Failed to load Cargo.lock");

        let fixer = Fixer::new(self.cargo_toml_path(), lockfile);

        let dry_run = self.dry_run;
        let dry_run_info = if dry_run { " (dry run)" } else { "" };

        status_ok!(
            "Fixing",
            "vulnerable dependencies in `{}`{}",
            self.cargo_toml_path().display(),
            dry_run_info
        );

        for vulnerability in &report.vulnerabilities.list {
            let results = fixer.fix(vulnerability, dry_run);
            for outcome in results.outcomes {
                match outcome.output {
                    Ok(_) => todo!(),
                    Err(e) => status_warn!(
                        "Failed to run `cargo update` for package {}: {}",
                        &outcome.package.name,
                        e
                    ),
                }
            }
        }
    }
}
