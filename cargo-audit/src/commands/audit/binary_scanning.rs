//! The `cargo audit bin` subcommand

use std::{path::PathBuf, process::exit};

use clap::Parser;

use crate::auditor::Auditor;

#[cfg(feature = "binary-scanning")]
/// The `cargo audit` subcommand
#[derive(Clone, Default, Debug, Parser)]
pub struct BinCommand {
    /// Paths to the binaries to be scanned
    #[arg(
        value_parser,
        required = true,
        help = "Paths to the binaries to be scanned"
    )]
    binary_paths: Vec<PathBuf>,
}

impl BinCommand {
    pub fn run(&self, auditor: &mut Auditor) {
        let report = auditor.audit_binaries(&self.binary_paths);
        if report.vulnerabilities_found {
            exit(1)
        } else if report.errors_encountered {
            exit(2)
        } else {
            exit(0)
        }
    }
}
