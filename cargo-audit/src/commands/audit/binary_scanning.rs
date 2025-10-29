//! The `cargo audit bin` subcommand

use std::{path::PathBuf, process::exit};

use abscissa_core::{Application, Command, Runnable};
use clap::Parser;

use crate::{application::APP, auditor::Auditor};

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
        Auditor::new(&APP.get().unwrap().config())
    }
}
