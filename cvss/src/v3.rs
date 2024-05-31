//! Common Vulnerability Scoring System (v3.1)
//!
//! <https://www.first.org/cvss/specification-document>

// TODO(tarcieri): Environmental and Temporal Metrics

pub mod base;
pub mod cvss;
pub mod environmental;
mod score;
pub mod temporal;

pub use self::{base::Base, score::Score};
