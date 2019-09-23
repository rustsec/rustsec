//! Application-local prelude: conveniently import types/functions/macros
//! which are generally useful and should be available everywhere.

/// Application state accessors
pub use crate::application::{app_config, app_reader, app_writer};

/// Commonly used Abscissa traits
pub use abscissa_core::{Application, Command, Runnable};

/// Error macros
pub use abscissa_core::{ensure, fail, fatal};

/// Logging macros
pub use abscissa_core::log::{debug, error, info, log, log_enabled, trace, warn};

/// Status macros
pub use abscissa_core::{
    status_attr_err, status_attr_ok, status_err, status_info, status_ok, status_warn,
};
