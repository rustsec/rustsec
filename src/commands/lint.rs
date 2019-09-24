//! `rustsec-admin lint` subcommand

use abscissa_core::{Command, Runnable};
use gumdrop::Options;
use std::path::{Path, PathBuf};

/// `rustsec-admin lint` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct LintCmd {
    /// Path to the advisory database
    #[options(free, help = "filesystem path to the RustSec advisory DB git repo")]
    path: Vec<PathBuf>,
}

impl Runnable for LintCmd {
    fn run(&self) {
        let repo_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => Self::print_usage_and_exit(&[]),
        };

        crate::linter::lint_advisories(repo_path);
    }
}
