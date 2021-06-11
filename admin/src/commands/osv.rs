//! `rustsec-admin osv` subcommand
//!
//! Exports all advisories to the OSV format defined at
//! https://github.com/google/osv

#![allow(warnings)] //TODO
#![warn(warnings)] //TODO

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use abscissa_core::{status_err, Command, Options, Runnable};

use crate::osv_export::OsvExporter;

#[derive(Command, Debug, Default, Options)]
pub struct OsvCmd {
    /// Path to the output directory
    #[options(
        required,
        short = "o",
        long = "out-dir",
        help = "filesystem directory where OSV JSON files will be written"
    )]
    out_dir: PathBuf,
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

        let exporter = OsvExporter::new(repo_path).unwrap_or_else(|e| {
            status_err!(
                "could not open advisory DB at '{}': {}",
                repo_path.display(),
                e
            );
            exit(1);
        });
        let out_path = &self.out_dir;
        exporter.export_all(out_path).unwrap_or_else(|e| {
            status_err!("failed not export to '{}': {}", out_path.display(), e);
            exit(1);
        });
    }
}
