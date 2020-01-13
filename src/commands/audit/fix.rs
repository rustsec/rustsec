//! The `cargo audit fix` subcommand

use crate::{auditor::Auditor, prelude::*};
use abscissa_core::{Command, Runnable};
use gumdrop::Options;
use rustsec::fixer::Fixer;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Command, Default, Debug, Options)]
pub struct FixCommand {
    /// Get help information
    #[options(short = "h", long = "help", help = "output help information and exit")]
    help: bool,

    /// Path to `Cargo.lock`
    #[options(short = "f", long = "file", help = "Cargo lockfile to inspect")]
    file: Option<PathBuf>,

    /// Perform a dry run
    #[options(no_short, long = "dry-run", help = "perform a dry run for the fix")]
    dry_run: Option<bool>,
}

impl FixCommand {
    /// Initialize `Auditor`
    pub fn auditor(&self) -> Auditor {
        let config = app_config();
        Auditor::new(&config)
    }

    /// Locate `Cargo.toml`
    // TODO(tarcieri): ability to specify path
    pub fn cargo_toml_path(&self) -> PathBuf {
        PathBuf::from("Cargo.toml")
    }

    /// Locate `Cargo.lock`
    pub fn cargo_lock_path(&self) -> Option<&Path> {
        self.file.as_ref().map(PathBuf::as_path)
    }
}

impl Runnable for FixCommand {
    fn run(&self) {
        if self.help {
            Self::print_usage_and_exit(&[]);
        }

        let report = self.auditor().audit(self.cargo_lock_path());

        if report.vulnerabilities.list.is_empty() {
            exit(0);
        }

        let mut fixer = Fixer::new(self.cargo_toml_path()).unwrap_or_else(|e| {
            status_err!(
                "couldn't load manifest from {}: {}",
                self.cargo_toml_path().display(),
                e
            );
            exit(1);
        });

        let dry_run = self.dry_run.unwrap_or_default();
        let dry_run_info = if dry_run { " (dry run)" } else { "" };

        status_ok!(
            "Fixing",
            "vulnerable dependencies in `{}`{}",
            self.cargo_toml_path().display(),
            dry_run_info
        );

        for vulnerability in &report.vulnerabilities.list {
            if let Err(e) = fixer.fix(vulnerability, dry_run) {
                status_warn!("{}", e);
            }
        }
    }
}
