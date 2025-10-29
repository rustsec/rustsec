//! Main entry point for `cargo audit`

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use std::process;

use abscissa_core::{Application, Runnable, Shutdown, application::fatal_error};
use cargo_audit::{
    application::{APP, CargoAuditApplication},
    commands::CargoAuditCommand,
};
use clap::Parser;

fn main() {
    // Parse command line options
    let command = CargoAuditCommand::parse();

    // Initialize application
    let mut app = CargoAuditApplication::default();
    app.init(&command).unwrap_or_else(|e| fatal_error(&app, &e));
    let app = APP.get_or_init(|| app);

    // Run the command
    command.run();

    // Exit gracefully
    let components = app.state().components();

    if let Err(e) = components.shutdown(app, Shutdown::Graceful) {
        fatal_error(app, &e)
    }

    process::exit(0);
}
