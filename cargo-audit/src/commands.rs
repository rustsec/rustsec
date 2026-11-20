//! `cargo audit` subcommands

use crate::{config::AuditConfig, error::Error};
use clap::Parser;
use std::{ops::Deref, process::ExitCode};

mod audit;
pub use audit::AuditCommand;

/// Name of the configuration file
///
/// This file allows setting some default auditing options.
pub const CONFIG_FILE: &str = "audit.toml";

/// `cargo audit` subcommands (presently only `audit`)
#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
pub enum CargoAuditSubCommand {
    /// The `cargo audit` subcommand
    #[command(about = "Audit Cargo.lock files for vulnerable crates")]
    Audit(AuditCommand),
}

impl CargoAuditSubCommand {
    /// Run the subcommand with the given configuration
    pub fn run(&self, config: &AuditConfig) -> Result<ExitCode, Error> {
        match self {
            Self::Audit(cmd) => cmd.run(config),
        }
    }
}

/// `cargo audit` entrypoint
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CargoAuditCommand {
    #[command(subcommand)]
    cmd: CargoAuditSubCommand,

    /// Increase verbosity setting
    #[arg(short = 'v', long, help = "Increase verbosity")]
    pub verbose: bool,
}

impl CargoAuditCommand {
    /// Run the command with the given configuration
    pub fn run(&self, config: &AuditConfig) -> Result<ExitCode, Error> {
        self.cmd.run(config)
    }

    /// Override loaded config with explicit command-line arguments
    pub fn process_config(&self, config: AuditConfig) -> Result<AuditConfig, Error> {
        match &self.cmd {
            CargoAuditSubCommand::Audit(cmd) => cmd.override_config(config),
        }
    }
}

impl Deref for CargoAuditCommand {
    type Target = AuditCommand;

    fn deref(&self) -> &AuditCommand {
        match &self.cmd {
            CargoAuditSubCommand::Audit(cmd) => cmd,
        }
    }
}
