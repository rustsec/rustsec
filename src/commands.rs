//! Cargo Audit Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod audit;

use self::audit::AuditCommand;
use crate::config::CargoAuditConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Options, Runnable};
use std::path::PathBuf;

/// Name of the configuration file (located in `~/.cargo`)
pub const CONFIG_FILE: &str = "audit.toml";

/// Cargo Audit Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum CargoAuditCommand {
    /// The `cargo audit` subcommand
    #[options(help = "Audit Cargo.lock files for vulnerable crates")]
    Audit(AuditCommand),
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<CargoAuditConfig> for CargoAuditCommand {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        let filename = home::cargo_home()
            .ok()
            .map(|cargo_home| cargo_home.join(CONFIG_FILE))?;

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Override loaded config using
    fn process_config(&self, config: CargoAuditConfig) -> Result<CargoAuditConfig, FrameworkError> {
        match self {
            CargoAuditCommand::Audit(cmd) => cmd.override_config(config),
        }
    }
}
