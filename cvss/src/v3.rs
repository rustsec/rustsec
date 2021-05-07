//! Common Vulnerability Scoring System (v3.1)
//!
//! <https://www.first.org/cvss/specification-document>

// TODO(tarcieri): Environmental and Temporal Metrics

pub mod base;
pub mod metric;
pub mod score;

pub use self::{base::Base, metric::Metric, score::Score};
