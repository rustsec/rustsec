//! CVSS v2.0 metrics.

use crate::{Error, Result};
use alloc::borrow::ToOwned;
use core::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

/// Trait for CVSS v2.0 metrics.
pub trait Metric: Copy + Clone + Debug + Display + Eq + FromStr + Ord {
    /// [`MetricType`] of this metric.
    const TYPE: MetricType;

    /// Get the name of this metric.
    fn name() -> &'static str {
        Self::TYPE.name()
    }

    /// Get CVSS v3.1 score for this metric.
    fn score(self) -> f64;

    /// Get `str` describing this metric's value
    fn as_str(self) -> &'static str;
}

/// Enum over all of the available v3.1 metrics.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum MetricType {
    /// Access Complexity (AC)
    AC,

    /// Access Vector (AV)
    AV,

    /// Authentication (Au)
    Au,
}

impl MetricType {
    /// Get the name of this metric (i.e. acronym)
    pub fn name(self) -> &'static str {
        match self {
            Self::AC => "AC",
            Self::Au => "Au",
            Self::AV => "AV",
        }
    }

    /// Get a description of this metric.
    pub fn description(self) -> &'static str {
        match self {
            Self::AC => "Access Complexity",
            Self::Au => "Authentication",
            Self::AV => "Access Vector",
        }
    }
}

impl Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for MetricType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "AC" => Ok(Self::AC),
            "Au" => Ok(Self::Au),
            "AV" => Ok(Self::AV),
            _ => Err(Error::UnknownMetric { name: s.to_owned() }),
        }
    }
}
