//! Application entrypoint for `cargo audit`

use std::{env, fs, path::PathBuf, process::ExitCode};

use clap::Parser;

use crate::{
    commands::{CONFIG_FILE, CargoAuditCommand},
    config::AuditConfig,
    error::{Error, ErrorKind},
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
    let config = load_config(&command)?;

    // Run the command
    command.run(&config)
}

/// Load `audit.toml` (if it exists), overriding it with command-line arguments
fn load_config(command: &CargoAuditCommand) -> Result<AuditConfig, Error> {
    // Check if the config file exists, and if it does not, ignore it.
    //
    // The order of precedence for which config file to use is:
    // 1. The current project's `.cargo` configuration directory.
    // 2. The current user's home directory configuration.

    let mut base = None;
    if let Ok(home) = home::cargo_home() {
        let home_config = home.join(CONFIG_FILE);
        if home_config.exists() {
            base = Some(home.join(CONFIG_FILE));
        }
    };

    let project_config_filename = PathBuf::from("./.cargo").join(CONFIG_FILE);
    if project_config_filename.exists() {
        base = Some(project_config_filename);
    }

    let config = match base {
        Some(path) => {
            let toml_string = fs::read_to_string(&path).map_err(|e| {
                ErrorKind::Config.context(format!("couldn't read {}: {e}", path.display()))
            })?;
            toml::from_str(&toml_string).map_err(|e| ErrorKind::Parse.context(e))?
        }
        None => AuditConfig::default(),
    };

    command.process_config(config)
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
