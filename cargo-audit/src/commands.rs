//! `cargo audit` subcommands

#[cfg(feature = "binary-scanning")]
use crate::commands::audit::AuditSubcommand;
use crate::{config::AuditConfig, error::Error};
use clap::Parser;
use std::{ops::Deref, process::ExitCode};

mod audit;
pub use audit::AuditCommand;

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
    pub fn run(self, config: AuditConfig) -> Result<ExitCode, Error> {
        let Self::Audit(cmd) = self;
        match cmd.subcommand {
            #[cfg(feature = "binary-scanning")]
            Some(AuditSubcommand::Bin(bin)) => bin.run(config),
            #[cfg(feature = "fix")]
            Some(AuditSubcommand::Fix(fix)) => fix.run(config),
            Some(AuditSubcommand::UpdateDb(update_db)) => update_db.run(config),
            None => cmd.run(config),
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
    pub fn run(self, config: AuditConfig) -> Result<ExitCode, Error> {
        self.cmd.run(config)
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
