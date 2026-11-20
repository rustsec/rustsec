//! Application entrypoint for `cargo audit`

use std::{
    env,
    io::{self, IsTerminal},
    ops::Deref,
    process::ExitCode,
};

use clap::Parser;

#[cfg(feature = "binary-scanning")]
use crate::commands::AuditSubcommand;
use crate::{
    commands::AuditCommand,
    config::AuditConfig,
    error::Error,
    terminal::{self, ColorChoice},
};

/// Boot the application, parsing subcommand and options from command-line
/// arguments, and terminating when complete.
pub fn boot() -> Result<ExitCode, Error> {
    // Parse command line options
    let command = CargoAuditCommand::parse();

    // Initialize the terminal and tracing subsystems
    let color_choice = match command.color {
        Some(color) => color.into(),
        None => match env::var("CARGO_TERM_COLOR") {
            Ok(e) if e == "always" => ColorChoice::Always,
            Ok(e) if e == "never" => ColorChoice::Never,
            Ok(e) if e == "auto" => ColorChoice::Auto,
            _ => match io::stdout().is_terminal() {
                true => ColorChoice::Auto,
                false => ColorChoice::Never,
            },
        },
    };

    terminal::init(color_choice);
    init_tracing(&command, color_choice);

    // Load configuration
    let config = AuditConfig::load(&command)?;

    // Run the command
    let CargoAuditSubCommand::Audit(cmd) = command.cmd;
    match cmd.subcommand {
        #[cfg(feature = "binary-scanning")]
        Some(AuditSubcommand::Bin(bin)) => bin.run(config),
        #[cfg(feature = "fix")]
        Some(AuditSubcommand::Fix(fix)) => fix.run(config),
        Some(AuditSubcommand::UpdateDb(update_db)) => update_db.run(config),
        None => cmd.run(config),
    }
}

/// `cargo audit` subcommands (presently only `audit`)
#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
pub enum CargoAuditSubCommand {
    /// The `cargo audit` subcommand
    #[command(about = "Audit Cargo.lock files for vulnerable crates")]
    Audit(AuditCommand),
}

/// `cargo audit` entrypoint
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CargoAuditCommand {
    /// Subcommand to execute (only `audit` is currently supported)
    #[command(subcommand)]
    pub cmd: CargoAuditSubCommand,

    /// Increase verbosity setting
    #[arg(short = 'v', long, help = "Increase verbosity")]
    pub verbose: bool,
}

impl Deref for CargoAuditCommand {
    type Target = AuditCommand;

    fn deref(&self) -> &AuditCommand {
        match &self.cmd {
            CargoAuditSubCommand::Audit(cmd) => cmd,
        }
    }
}

/// Initialize the `tracing` subsystem from command-line options
fn init_tracing(command: &CargoAuditCommand, color_choice: ColorChoice) {
    let filter = match command.verbose {
        true => "debug".to_owned(),
        false => env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()),
    };

    tracing_subscriber::fmt()
        .with_ansi(color_choice != ColorChoice::Never)
        .with_env_filter(filter)
        .try_init()
        .expect("tracing subsystem failed to initialize");
}
