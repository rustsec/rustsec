//! Common Vulnerability Scoring System (v3.1)
//!
//! <https://www.first.org/cvss/specification-document>

// TODO(tarcieri): Environmental and Temporal Metrics

#[cfg(feature = "v3")]
pub mod base;

pub mod metric;

#[cfg(feature = "v3")]
mod score;
mod metric;

<<<<<<< HEAD
#[cfg(feature = "v3")]
pub use self::{
    base::Base,
    metric::{Metric, MetricType},
    score::Score,
};
=======
pub use self::{base::Base, score::Score, metric::Metric, metric::MetricType};
>>>>>>> f3ae091 (v2 metrics and Au)
