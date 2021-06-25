//! `rustsec-admin osv` subcommand
//!
//! Exports all advisories to the OSV format defined at
//! https://github.com/google/osv

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
        free,
        help = "filesystem directory where OSV JSON files will be written"
    )]
    path: Vec<PathBuf>,
}

impl Runnable for OsvCmd {
    fn run(&self) {
        let out_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => Self::print_usage_and_exit(&[]),
        };

        let exporter = OsvExporter::new().unwrap_or_else(|e| {
            status_err!("Failed to fetch the advisory database: {}", e);
            exit(1);
        });
        exporter.export_all(out_path).unwrap_or_else(|e| {
            status_err!("failed not export to '{}': {}", out_path.display(), e);
            exit(1);
        });
    }
}
