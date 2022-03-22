//! `rustsec-admin` Abscissa [`Application`] type.
//!
//! <https://github.com/iqlusioninc/abscissa/>

use std::ops::Deref;
use std::sync::Arc;

use crate::{commands::AdminCmd, config::AppConfig};
use abscissa_core::{
    application::{self, AppCell},
    config::{self, CfgCell},
    trace, Application, FrameworkError, StandardPaths,
};

/// Application state
pub static APPLICATION: AppCell<AdminApp> = AppCell::new();

/// Obtain a read-only (multi-reader) lock on the application state.
///
/// Panics if the application state has not been initialized.
pub fn app_reader() -> &'static AdminApp {
    APPLICATION.deref()
}

/// Obtain an exclusive mutable lock on the application state.
pub fn app_writer() -> &'static AdminApp {
    APPLICATION.deref()
}

/// Obtain a read-only (multi-reader) lock on the application configuration.
///
/// Panics if the application configuration has not been loaded.
pub fn app_config() -> config::Reader<AppConfig> {
    APPLICATION.config.read()
}

/// `rustsec-admin` Abscissa [`Application`] type
#[derive(Debug, Default)]
pub struct AdminApp {
    /// Application configuration.
    config: CfgCell<AppConfig>,

    /// Application state.
    state: application::State<Self>,
}

impl Application for AdminApp {
    /// Entrypoint command for this application.
    type Cmd = AdminCmd;

    /// Application configuration.
    type Cfg = AppConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> Arc<AppConfig> {
        self.config.read()
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Register all components used by this application.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.state.components_mut().register(components)
    }

    /// Post-configuration lifecycle callback.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        self.state.components_mut().after_config(&config)?;
        self.config.set_once(config);
        Ok(())
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &AdminCmd) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
