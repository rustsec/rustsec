//! Configuration file
//!
//! This is a placeholder since this command doesn't presently have one.

use abscissa_core::Config;
use serde::{Deserialize, Serialize};

/// Configuration File
#[derive(Clone, Config, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {}
