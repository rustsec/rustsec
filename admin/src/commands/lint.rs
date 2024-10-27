//! `rustsec-admin lint` subcommand

use crate::{linter::Linter, prelude::*};
use abscissa_core::{Command, Runnable};
use clap::Parser;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

/// `rustsec-admin lint` subcommand
#[derive(Command, Debug, Default, Parser)]
pub struct LintCmd {
    /// Path to the advisory database
    #[arg(
        num_args = 1..,
        help = "filesystem path to the RustSec advisory DB git repo"
    )]
    path: Vec<PathBuf>,

    #[arg(long, help = "Skip name check comma separated crates list")]
    skip_namecheck: Option<String>,
}

impl Runnable for LintCmd {
    fn run(&self) {
        let repo_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => unreachable!(),
        };

        let linter = Linter::new(repo_path, self.skip_namecheck.to_owned()).unwrap_or_else(|e| {
            status_err!(
                "error loading advisory DB repo from {}: {}",
                repo_path.display(),
                e
            );

            exit(1);
        });

        let advisories = linter.advisory_db().iter();

        // Ensure we're parsing some advisories
        if advisories.len() == 0 {
            status_err!("no advisories found!");
            exit(1);
        }

        status_ok!(
            "Loaded",
            "{} security advisories (from {})",
            advisories.len(),
            repo_path.display()
        );

        let invalid_advisory_count = linter.lint().unwrap_or_else(|e| {
            status_err!("error linting advisory DB {}: {}", repo_path.display(), e);

            exit(1);
        });

        if invalid_advisory_count == 0 {
            status_ok!("Success", "all advisories are well-formed");
        } else {
            status_err!("{} advisories contain errors!", invalid_advisory_count);
            exit(1);
        }
    }
}
