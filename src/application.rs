//! Abscissa application for `cargo audit`
//!
//! <https://docs.rs/abscissa_core>

use crate::{commands::CargoAuditCommand, config::AuditConfig};
use abscissa_core::{
    application::{self, AppCell},
    config,
    terminal::ColorChoice,
    trace, Application, EntryPoint, FrameworkError, StandardPaths,
};

/// Application state
pub static APPLICATION: AppCell<CargoAuditApplication> = AppCell::new();

/// Obtain a read-only (multi-reader) lock on the application state.
///
/// Panics if the application state has not been initialized.
pub fn app_reader() -> application::lock::Reader<CargoAuditApplication> {
    APPLICATION.read()
}

/// Obtain an exclusive mutable lock on the application state.
pub fn app_writer() -> application::lock::Writer<CargoAuditApplication> {
    APPLICATION.write()
}

/// Obtain a read-only (multi-reader) lock on the application configuration.
///
/// Panics if the application configuration has not been loaded.
pub fn app_config() -> config::Reader<CargoAuditApplication> {
    config::Reader::new(&APPLICATION)
}

/// `cargo audit` application
#[derive(Debug)]
pub struct CargoAuditApplication {
    /// Application configuration.
    config: Option<AuditConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
///
/// By default no configuration is loaded, and the framework state is
/// initialized to a default, empty state (no components, threads, etc).
impl Default for CargoAuditApplication {
    fn default() -> Self {
        Self {
            config: None,
            state: application::State::default(),
        }
    }
}

impl Application for CargoAuditApplication {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint<CargoAuditCommand>;

    /// Application configuration.
    type Cfg = AuditConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> &AuditConfig {
        self.config.as_ref().expect("config not loaded")
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Borrow the application state mutably.
    fn state_mut(&mut self) -> &mut application::State<Self> {
        &mut self.state
    }

    /// Register all components used by this application.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.state.components.register(components)
    }

    /// Post-configuration lifecycle callback.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        self.state.components.after_config(&config)?;
        self.config = Some(config);
        Ok(())
    }

    /// Color configuration for this application.
    fn term_colors(&self, entrypoint: &EntryPoint<CargoAuditCommand>) -> ColorChoice {
        entrypoint
            .command
            .as_ref()
            .and_then(|cmd| cmd.color_config())
            .unwrap_or_else(|| ColorChoice::Auto)
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &EntryPoint<CargoAuditCommand>) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
