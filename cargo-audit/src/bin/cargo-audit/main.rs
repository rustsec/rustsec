//! Main entry point for `cargo audit`

#![deny(warnings, missing_docs, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::{Application, Configurable, Runnable, config::Override, terminal::ColorChoice};
use cargo_audit::{
    application::CargoAuditApplication,
    commands::{CargoAuditCommand, CargoAuditSubCommand},
    config::AuditConfig,
};
use clap::Parser;
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line options
    let command = CargoAuditCommand::parse();

    // Initialize application
    let mut app = CargoAuditApplication::default();
    if command.term_colors() != ColorChoice::Never {
        color_eyre::install()?;
    }

    LogTracer::init()?;

    // Construct a tracing subscriber with the supplied filter and enable reloading.
    let subscriber = FmtSubscriber::builder()
        .with_ansi(match command.term_colors() {
            ColorChoice::Always => true,
            ColorChoice::AlwaysAnsi => true,
            ColorChoice::Auto => true,
            ColorChoice::Never => false,
        })
        .with_env_filter(match command.verbose {
            true => "debug".to_owned(),
            false => std::env::var("RUST_LOG")
                .unwrap_or("info".to_owned())
                .into(),
        })
        .finish();

    // Now set it as the global tracing subscriber and save the handle.
    tracing::subscriber::set_global_default(subscriber)?;

    // Load configuration
    let config = match command.config_path() {
        Some(path) => app.load_config(&path)?,
        None => AuditConfig::default(),
    };

    // Fire callback regardless of whether any config was loaded to
    // in order to signal state in the application lifecycle
    let config = match &command.cmd {
        CargoAuditSubCommand::Audit(cmd) => cmd.override_config(config)?,
    };

    // Run the command
    app.config.set_once(config);
    command.run();
    Ok(())
}
