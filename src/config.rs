//! The `~/.cargo/audit.toml` configuration file

use abscissa_core::Config;
use serde::{Deserialize, Serialize};

/// `cargo audit` configuration:
///
/// An optional TOML config file located in `~/.cargo/audit.toml`
#[derive(Clone, Config, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CargoAuditConfig {
    /// An example configuration section
    pub display: DisplayConfig,
}

/// Options for how audit reports are displayed
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayConfig {
    /// Should we display colors?
    pub color: Option<String>,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            color: Some("auto".to_owned()),
        }
    }
}
