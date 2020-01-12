//! The `~/.cargo/audit.toml` configuration file

use rustsec::database::package_scope::{PackageScope, PackageSource};
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuditConfig {
    /// Advisory-related configuration
    pub advisories: AdvisoryConfig,

    /// Advisory Database configuration
    pub database: DatabaseConfig,

    /// Output configuration
    pub output: OutputConfig,

    /// Target-related configuration
    pub target: TargetConfig,

    /// Packages-related configuration
    pub packages: PackageConfig,
}

impl AuditConfig {
    /// Get audit report settings from the configuration
    pub fn report_settings(&self) -> report::Settings {
        let mut settings = rustsec::report::Settings::default();
        settings.ignore = self.advisories.ignore.clone();
        settings.severity = self.advisories.severity_threshold;
        settings.target_arch = self.target.arch;
        settings.target_os = self.target.os;

        if let Some(source) = &self.packages.source {
            settings.package_scope = Some(PackageScope::from_source(source.clone()));
        }

        if let Some(informational_warnings) = &self.advisories.informational_warnings {
            settings.informational_warnings = informational_warnings.clone();
        } else {
            // Alert for unmaintained packages by default
            settings.informational_warnings = vec![advisory::Informational::Unmaintained];
        }

        settings
    }
}

/// Advisory-related configuration.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisoryConfig {
    /// Ignore advisories for the given IDs
    #[serde(default)]
    pub ignore: Vec<advisory::Id>,

    /// Warn for the given types of informational advisories
    pub informational_warnings: Option<Vec<advisory::Informational>>,

    /// CVSS Qualitative Severity Rating Scale threshold to alert at.
    ///
    /// Vulnerabilities with explicit CVSS info which have a severity below
    /// this threshold will be ignored.
    pub severity_threshold: Option<advisory::Severity>,
}

/// Advisory Database configuration.
///
/// The advisory database is stored in a Git repository. This section of the
/// configuration stores settings related to it.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfig {
    /// Path to the local copy of advisory database's git repo (default: ~/.cargo/advisory-db)
    pub path: Option<PathBuf>,

    /// URL to the advisory database's git repo (default: https://github.com/RustSec/advisory-db)
    pub url: Option<String>,

    /// Perform a `git fetch` before auditing (default: true)
    pub fetch: bool,

    /// Allow a stale advisory database? (i.e. one which hasn't been updated in 90 days)
    pub stale: bool,
}

/// Output configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputConfig {
    /// Should colors be displayed?
    pub color: Option<String>,

    /// Disallow any warning advisories
    #[serde(default)]
    pub deny_warnings: bool,

    /// Output format to use
    #[serde(default)]
    pub format: OutputFormat,

    /// Enable quiet mode
    pub quiet: bool,

    /// Show inverse dependency trees along with advisories (default: true)
    pub show_tree: Option<bool>,
}

impl OutputConfig {
    /// Is quiet mode enabled?
    pub fn is_quiet(&self) -> bool {
        self.quiet || self.format == OutputFormat::Json
    }
}

/// Output format
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

/// Target configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetConfig {
    /// Target architecture to find vulnerabilities for
    pub arch: Option<Arch>,

    /// Target OS to find vulnerabilities for
    pub os: Option<OS>,
}

/// Packages configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PackageConfig {
    /// Package scope which should be considered for querying for vulnerabilities.
    pub source: Option<PackageSource>,
}
