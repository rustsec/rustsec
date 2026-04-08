//! The `cargo audit bin` subcommand

use crate::{auditor::Auditor, prelude::*};
use clap::Parser;
use std::{path::PathBuf, process::exit};

#[cfg(feature = "binary-scanning")]
/// The `cargo audit` subcommand
#[derive(Command, Clone, Default, Debug, Parser)]
#[command()]
pub struct BinCommand {
    /// Maximum binary size in bytes to read
    #[arg(
        long = "max-binary-size",
        value_name = "BYTES",
        help = "Maximum binary size in bytes to read (default: 100MB; use 0 for unlimited)"
    )]
    max_binary_size: Option<u64>,

    /// Maximum audit data size in bytes to parse
    #[arg(
        long = "audit-data-size-limit",
        value_name = "BYTES",
        help = "Maximum audit data size in bytes to parse (default: 8MB)"
    )]
    audit_data_size_limit: Option<usize>,

    /// Paths to the binaries to be scanned
    #[arg(
        value_parser,
        required = true,
        help = "Paths to the binaries to be scanned"
    )]
    binary_paths: Vec<PathBuf>,
}

impl Runnable for BinCommand {
    fn run(&self) {
        let mut auditor = self.auditor();
        auditor.set_binary_scan_limits(self.max_binary_size, self.audit_data_size_limit);
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

impl BinCommand {
    /// Initialize `Auditor`
    pub fn auditor(&self) -> Auditor {
        Auditor::new(&APP.config())
    }
}
