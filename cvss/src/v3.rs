//! Common Vulnerability Scoring System (v3.1)
//!
//! <https://www.first.org/cvss/specification-document>

// TODO(tarcieri): Environmental and Temporal Metrics

pub mod metric;

#[cfg(feature = "v3")]
mod score;

#[cfg(feature = "v3")]
mod vector;

#[cfg(feature = "v3")]
pub use self::{
    metric::{Metric, MetricType},
    score::Score,
    vector::Vector,
};
