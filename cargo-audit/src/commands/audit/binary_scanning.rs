//! The `cargo audit bin` subcommand

use crate::{auditor::Auditor, prelude::*};
use clap::Parser;
use std::{path::PathBuf, process::exit};

#[cfg(feature = "binary-scanning")]
/// The `cargo audit` subcommand
#[derive(Command, Clone, Default, Debug, Parser)]
#[command()]
pub struct BinCommand {
    /// Paths to the binaries to be scanned
    #[arg(
        value_parser,
        required = true,
        help = "Paths to the binaries to be scanned"
    )]
    binary_paths: Vec<PathBuf>,

    /// Maximum size (bytes) of the input binary file that will be read into memory.
    /// Defaults to 512 MiB. Set to 0 to disable the limit (not recommended).
    #[arg(long, value_parser)]
    max_binary_size: Option<u64>,

    /// Maximum size (bytes) of embedded `cargo-auditable` metadata to extract.
    /// If unset, rustsec defaults to 8 MiB.
    #[arg(long, value_parser)]
    audit_data_size_limit: Option<usize>,
}

impl Runnable for BinCommand {
    fn run(&self) {
        let report = self.auditor().audit_binaries(&self.binary_paths);
        if report.vulnerabilities_found {
            exit(1)
        } else if report.errors_encountered {
            exit(2)
        } else {
            exit(0)
        }
    }
}

impl BinCommand {
    /// Initialize `Auditor`
    pub fn auditor(&self) -> Auditor {
        let mut config = APP.config().as_ref().clone();
        if let Some(limit) = self.max_binary_size {
            config.binary.max_binary_size = Some(limit);
        }
        if let Some(limit) = self.audit_data_size_limit {
            config.binary.audit_data_size_limit = Some(limit);
        }
        Auditor::new(&config)
    }
}
