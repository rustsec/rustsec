//! CVSS v3.1 metrics

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

/// Trait for CVSS v3.1 metrics
pub trait Metric: Copy + Clone + Debug + Display + Eq + FromStr + Ord {
    /// Name of the metric (e.g. `A`, `AC`, `AV`)
    const NAME: &'static str;

    /// Get CVSS v3.1 score for this metric
    fn score(self) -> f64;

    /// Get `str` describing this metric's value
    fn as_str(self) -> &'static str;
}
