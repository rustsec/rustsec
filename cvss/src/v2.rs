//! Common Vulnerability Scoring System (v2.0)
//!
//! <https://www.first.org/cvss/v2/guide>

pub mod base;
pub mod temporal;

mod score;
mod metric;
mod vector;

pub use self::{score::Score, metric::Metric, metric::MetricType, vector::Vector};