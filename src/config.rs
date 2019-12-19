//! Configuration file
//!
//! This is a placeholder since this command doesn't presently have one.

use serde::{Deserialize, Serialize};

/// Configuration File
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {}
