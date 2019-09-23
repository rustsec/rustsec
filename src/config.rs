//! The `~/.cargo/audit.toml` configuration file

use abscissa_core::Config;
use rustsec::{
    advisory,
    platforms::target::{Arch, OS},
    report,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// `cargo audit` configuration:
///
/// An optional TOML config file located in `~/.cargo/audit.toml`
#[derive(Clone, Config, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuditConfig {
    /// Path to the advisory database's git repo
    pub advisory_db_path: Option<PathBuf>,

    /// URL to the advisory database
    pub advisory_db_url: Option<String>,

    /// Allow a stale advisory database?
    pub allow_stale: bool,

    /// Should colors be displayed?
    pub color: Option<String>,

    /// Vector of advisories to be ignored
    #[serde(default)]
    pub ignore: Vec<advisory::Id>,

    /// Should we skip fetching the advisory database from git?
    pub no_fetch: bool,

    /// Output format to use
    #[serde(default)]
    pub output_format: OutputFormat,

    /// Enable quiet mode
    pub quiet: bool,

    /// Ignore advisories with a severity lower than this threshold
    pub severity_threshold: Option<advisory::Severity>,

    /// Show information about dependency trees with vulnerabilities
    pub show_dependency_tree: Option<bool>,

    /// Target architecture to find vulnerabilities for
    pub target_arch: Option<Arch>,

    /// Target OS to find vulnerabilities for
    pub target_os: Option<OS>,
}

impl AuditConfig {
    /// Get audit report settings from the configuration
    pub fn report_settings(&self) -> report::Settings {
        let mut settings = rustsec::report::Settings::default();

        // TODO(tarcieri): support for customizing informational advisory types to warn for
        settings.informational_warnings = vec![advisory::Informational::Unmaintained];
        settings.ignore = self.ignore.clone();
        settings.severity = self.severity_threshold;
        settings.target_arch = self.target_arch;
        settings.target_os = self.target_os;

        settings
    }
}

/// Output format
#[derive(Copy, Clone, Config, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OutputFormat {
    /// Display JSON
    #[serde(rename = "json")]
    Json,

    /// Display human-readable output to the terminal
    #[serde(rename = "terminal")]
    Terminal,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Terminal
    }
}
