//! CVSS v2.0 Base Metric Group - Confidentiality Impact (C)

use crate::Error;
use crate::v2::{Metric, MetricType};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Confidentiality Impact (C) - CVSS v2.0 Base Metric Group
///
/// Described in CVSS v2.0 Specification: Section 2.1.4:
/// <https://www.first.org/cvss/v2/guide#2-1-4-Confidentiality-Impact-C>
///
/// > This metric measures the impact on confidentiality of a successfully
/// > exploited vulnerability. Confidentiality refers to limiting information
/// > access and disclosure to only authorized users, as well as preventing
/// > access by, or disclosure to, unauthorized ones. The possible values for
/// > this metric are listed in Table 4. Increased confidentiality impact
/// > increases the vulnerability score.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ConfidentialityImpact {
    None,
    Partial,
    Complete,
}

impl Metric for ConfidentialityImpact {
    const TYPE: MetricType = MetricType::C;

    fn score(self) -> f64 {
        match self {
            ConfidentialityImpact::None => 0.0,
            ConfidentialityImpact::Partial => 0.275,
            ConfidentialityImpact::Complete => 0.660,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            ConfidentialityImpact::None => "N",
            ConfidentialityImpact::Partial => "P",
            ConfidentialityImpact::Complete => "C",
        }
    }
}

impl fmt::Display for ConfidentialityImpact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ConfidentialityImpact {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "N" => Ok(ConfidentialityImpact::None),
            "P" => Ok(ConfidentialityImpact::Partial),
            "C" => Ok(ConfidentialityImpact::Complete),
            _ => Err(Error::InvalidV2Metric {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
