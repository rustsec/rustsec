//! Main entry point for `cargo audit`

#![deny(warnings, missing_docs, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::{
    Application, Component, Configurable, Runnable, Shutdown,
    application::fatal_error,
    config::Override,
    terminal::component::Terminal,
    trace::{Config, Tracing},
};
use cargo_audit::{
    application::{APP, CargoAuditApplication},
    commands::{CargoAuditCommand, CargoAuditSubCommand},
    config::AuditConfig,
};
use clap::Parser;

fn main() {
    // Parse command line options
    let command = CargoAuditCommand::parse();

    // Initialize application
    let mut app = CargoAuditApplication::default();
    let terminal = Terminal::new(command.term_colors());
    let tracing = Tracing::new(
        match command.verbose {
            true => Config::verbose(),
            false => Config::default(),
        },
        command.term_colors(),
    )
    .expect("tracing subsystem failed to initialize");

    let components = vec![
        Box::new(terminal) as Box<dyn Component<CargoAuditApplication>>,
        Box::new(tracing),
    ];
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
