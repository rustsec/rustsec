//! Main entry point for `cargo audit`

#![deny(warnings, missing_docs, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::{
    Application, Component, Configurable, Runnable, Shutdown, application::fatal_error,
    config::Override, terminal::ColorChoice, terminal::component::Terminal,
};
use cargo_audit::{
    application::{APP, CargoAuditApplication},
    commands::{CargoAuditCommand, CargoAuditSubCommand},
    config::AuditConfig,
};
use clap::Parser;
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Parse command line options
    let command = CargoAuditCommand::parse();

    // Initialize application
    let mut app = CargoAuditApplication::default();
    let terminal = Terminal::new(command.term_colors());
    let components = vec![Box::new(terminal) as Box<dyn Component<CargoAuditApplication>>];

    if let Err(error) = LogTracer::init() {
        fatal_error(&app, &error);
    }

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
            false => std::env::var("RUST_LOG").unwrap_or("info".to_owned()),
        })
        .finish();

    // Now set it as the global tracing subscriber and save the handle.
    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        fatal_error(&app, &error)
    }

    if let Err(error) = app.state.components_mut().register(components) {
        fatal_error(&app, &error);
    };

    // Load configuration
    let config = match command.config_path() {
        Some(path) => match app.load_config(&path) {
            Ok(cfg) => cfg,
            Err(e) => fatal_error(&app, &e),
        },
        None => AuditConfig::default(),
    };

    // Fire callback regardless of whether any config was loaded to
    // in order to signal state in the application lifecycle
    let config = match &command.cmd {
        CargoAuditSubCommand::Audit(cmd) => match cmd.override_config(config) {
            Ok(cfg) => cfg,
            Err(e) => fatal_error(&app, &e),
        },
    };

    if let Err(error) = app.state.components_mut().after_config(&config) {
        fatal_error(&app, &error);
    }

    // Run the command
    app.config.set_once(config);
    let app = APP.get_or_init(|| app);
    command.run();

    // Exit gracefully
    let components = app.state().components();

    if let Err(e) = components.shutdown(app, Shutdown::Graceful) {
        fatal_error(app, &e)
    }
}
