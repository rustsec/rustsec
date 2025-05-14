//! CVSS v2.0 Base Metric Group - Availability Impact (C)

use crate::Error;
use crate::v2::{Metric, MetricType};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Availability Impact (C) - CVSS v2.0 Base Metric Group
///
/// Described in CVSS v2.0 Specification: Section 2.1.6:
/// <https://www.first.org/cvss/v2/guide#2-1-6-Availability-Impact-A>
///
/// > This metric measures the impact to availability of a successfully
/// > exploited vulnerability. Availability refers to the accessibility of
/// > information resources. Attacks that consume network bandwidth, processor
/// > cycles, or disk space all impact the availability of a system. The
/// > possible values for this metric are listed in Table 6. Increased
/// > availability impact increases the vulnerability score.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum AvailabilityImpact {
    None,
    Partial,
    Complete,
}

impl Metric for AvailabilityImpact {
    const TYPE: MetricType = MetricType::A;

    fn score(self) -> f64 {
        match self {
            AvailabilityImpact::None => 0.0,
            AvailabilityImpact::Partial => 0.275,
            AvailabilityImpact::Complete => 0.660,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            AvailabilityImpact::None => "N",
            AvailabilityImpact::Partial => "P",
            AvailabilityImpact::Complete => "C",
        }
    }
}

impl fmt::Display for AvailabilityImpact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for AvailabilityImpact {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "N" => Ok(AvailabilityImpact::None),
            "P" => Ok(AvailabilityImpact::Partial),
            "C" => Ok(AvailabilityImpact::Complete),
            _ => Err(Error::InvalidV2Metric {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
