//! `cargo audit` subcommands

mod audit;

use self::audit::AuditCommand;
use crate::config::AuditConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Options, Runnable};
use std::path::PathBuf;
use std::env;

/// Name of the configuration file (located in `~/.cargo`)
///
/// This file allows setting some default auditing options.
pub const CONFIG_FILE: &str = "audit.toml";

/// `cargo audit` subcommands (presently only `audit`)
#[derive(Command, Debug, Options, Runnable)]
pub enum CargoAuditCommand {
    /// The `cargo audit` subcommand
    #[options(help = "Audit Cargo.lock files for vulnerable crates")]
    Audit(AuditCommand),

    /// The `cargo fix` subcommand
    #[options(help = "Fix vulnerable packages where available")]
    Fix(AuditCommand),
}

impl Configurable<AuditConfig> for CargoAuditCommand {
    /// Location of `audit.toml` (if it exists)
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

    /// Override loaded config with explicit command-line arguments
    /// `fix` update vulnerabilities with `cargo-edit`
    /// `audit` match self
    fn process_config(&self, mut config: AuditConfig) -> Result<AuditConfig, FrameworkError> {
        let args: Vec<String> = env::args().collect();
        if args.iter().any(|arg| arg == "fix") {
            config.set_audit_and_fix_mode()
        }
        match self {
            CargoAuditCommand::Audit(cmd) => cmd.override_config(config),
            _ => Ok(config),
        }
    }
}
