//! Common Vulnerability Scoring System (v2.0)
//!
//! <https://www.first.org/cvss/v2/guide>

pub mod base;

mod score;

pub use self::{base::Base, score::Score};
