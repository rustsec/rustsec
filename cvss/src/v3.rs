//! Common Vulnerability Scoring System (v3.1)
//!
//! <https://www.first.org/cvss/specification-document>

#[cfg(feature = "v3")]
pub mod base;
pub mod metric;
#[cfg(feature = "v3")]
pub mod temporal;

#[cfg(feature = "v3")]
mod score;
#[cfg(feature = "v3")]
mod vector;

#[cfg(feature = "v3")]
pub use self::{
    base::Base,
    metric::{Metric, MetricType},
    score::Score,
    vector::Vector,
};
