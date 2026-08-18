//! CVSS v3.1 Environmental Metric Group - Modified Base Metrics

use core::{fmt, str::FromStr};

use crate::Error;
use crate::v3::{
    Metric, MetricType,
    metric::{
        BaseMetric, ModifiedMetric,
        base::{
            AttackComplexity, AttackVector, Availability, Confidentiality, Integrity,
            PrivilegesRequired, UserInteraction,
        },
    },
};

/// A CVSS v3.1 Modified Base Metric: either left unspecified ("Not Defined")
/// or overriding the corresponding Base metric's value.
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Modified<T> {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see the wrapped Base metric)
    Modified(T),
}

impl<T> Metric for Modified<T>
where
    T: BaseMetric + FromStr<Err = Error>,
{
    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }

    const TYPE: MetricType = T::MODIFIED_TYPE;
}

impl<T> ModifiedMetric<T> for Modified<T>
where
    T: BaseMetric + FromStr<Err = Error>,
{
    fn modified_score(self, base: Option<T>) -> f64 {
        match self {
            Self::Modified(v) => v.score(),
            Self::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
        }
    }
}

impl<T> fmt::Display for Modified<T>
where
    T: BaseMetric + FromStr<Err = Error>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl<T> FromStr for Modified<T>
where
    T: BaseMetric + FromStr<Err = Error>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "X" => Self::NotDefined,
            _ => Self::Modified(T::from_str(s)?),
        })
    }
}

impl Modified<PrivilegesRequired> {
    /// Calculate the Scoped Score for the Modified Privileges Required (MPR)
    /// metric.
    ///
    /// Its value depends on whether the scope of the
    /// [crate::v3::metric::environmental::ModifiedScope] (or
    /// [crate::v3::metric::base::Scope] base) metric has changed.
    pub fn scoped_score(self, scope_changed: bool, base: Option<PrivilegesRequired>) -> f64 {
        match self {
            Self::Modified(v) => v.scoped_score(scope_changed),
            Self::NotDefined => base.map(|b| b.scoped_score(scope_changed)).unwrap_or(0.0),
        }
    }
}

impl BaseMetric for AttackVector {
    const MODIFIED_TYPE: MetricType = MetricType::MAV;
}

impl BaseMetric for AttackComplexity {
    const MODIFIED_TYPE: MetricType = MetricType::MAC;
}

impl BaseMetric for PrivilegesRequired {
    const MODIFIED_TYPE: MetricType = MetricType::MPR;
}

impl BaseMetric for UserInteraction {
    const MODIFIED_TYPE: MetricType = MetricType::MUI;
}

impl BaseMetric for Confidentiality {
    const MODIFIED_TYPE: MetricType = MetricType::MC;
}

impl BaseMetric for Integrity {
    const MODIFIED_TYPE: MetricType = MetricType::MI;
}

impl BaseMetric for Availability {
    const MODIFIED_TYPE: MetricType = MetricType::MA;
}
