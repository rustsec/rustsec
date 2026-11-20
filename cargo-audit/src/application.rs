//! Application entrypoint for `cargo audit`

use std::{env, process::ExitCode};

use clap::Parser;

use crate::{
    commands::CargoAuditCommand,
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
    let color_choice = command.term_colors();
    terminal::init(color_choice);
    init_tracing(&command, color_choice);

    // Load configuration
    let config = AuditConfig::load(&command)?;

    // Run the command
    command.run(config)
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
