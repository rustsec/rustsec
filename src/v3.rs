//! Common Vulnerability Scoring System (v3.1)
//!
//! <https://www.first.org/cvss/specification-document>

// TODO(tarcieri): Environmental and Temporal Metrics

pub mod base;
pub mod metric;
pub mod score;

pub use self::{base::Base, metric::Metric, score::Score};

/// Current CVSS v3 version supported by this library
const CURRENT_VERSION: &str = "3.1";

/// Supported CVSS v3 versions
const SUPPORTED_VERSIONS: &[&str] = &["3.0", "3.1"];
