//! `rustsec-admin osv` subcommand
//!
//! Exports all advisories to the OSV format defined at
//! https://github.com/google/osv

#![allow(unused_variables)] //TODO

use std::path::{Path, PathBuf};

use abscissa_core::{Command, Options, Runnable};
#[derive(Command, Debug, Default, Options)]
pub struct OsvCmd {
    /// Path to the advisory database
    #[options(free, help = "filesystem path to the RustSec advisory DB git repo")]
    path: Vec<PathBuf>,
}

impl Runnable for OsvCmd {
    fn run(&self) {
        let repo_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => Self::print_usage_and_exit(&[]),
        };

    }
}
