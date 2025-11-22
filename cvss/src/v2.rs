//! Common Vulnerability Scoring System (v2.0)
//!
//! <https://www.first.org/cvss/v2/guide>

pub mod base;

mod score;
mod metric;

pub use self::{base::Base, score::Score, metric::Metric, metric::MetricType};
