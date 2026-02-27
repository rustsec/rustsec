//! CVSS v3.1 Environmental Metric Group - Confidentiality Requirements (CR)

use crate::{Error, Metric, MetricType};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Confidentiality Requirements (CR) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.1:
/// <https://www.first.org/cvss/v3-1/specification-document#4-1-Security-Requirements-CR-IR-AR>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ConfidentialityRequirement {
    /// Not Defined (X)
    /// > Assigning this value indicates there is insufficient information to
    /// > choose one of the other values, and has no impact on the overall
    /// > Environmental Score, i.e., it has the same effect on scoring as
    /// > assigning Medium.
    NotDefined,

    /// High (H)
    /// > Loss of Confidentiality is likely to have a catastrophic adverse
    /// > effect on the organization or individuals associated with the
    /// > organization (e.g., employees, customers).
    High,

    /// Medium (M)
    /// > Loss of Confidentiality is likely to have a serious adverse effect
    /// > on the organization or individuals associated with the organization
    /// > (e.g., employees, customers).
    Medium,

    /// Low (L)
    /// > Loss of Confidentiality is likely to have only a limited adverse
    /// > effect on the organization or individuals associated with the
    /// > organization (e.g., employees, customers).
    Low,
}

impl Metric for ConfidentialityRequirement {
    const TYPE: MetricType = MetricType::CR;

    fn score(self) -> f64 {
        match self {
            ConfidentialityRequirement::NotDefined => 1.0,
            ConfidentialityRequirement::High => 1.5,
            ConfidentialityRequirement::Medium => 1.0,
            ConfidentialityRequirement::Low => 0.5,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            ConfidentialityRequirement::NotDefined => "X",
            ConfidentialityRequirement::High => "H",
            ConfidentialityRequirement::Medium => "M",
            ConfidentialityRequirement::Low => "L",
        }
    }
}

impl fmt::Display for ConfidentialityRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ConfidentialityRequirement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "X" => Ok(ConfidentialityRequirement::NotDefined),
            "L" => Ok(ConfidentialityRequirement::Low),
            "M" => Ok(ConfidentialityRequirement::Medium),
            "H" => Ok(ConfidentialityRequirement::High),
            _ => Err(Error::InvalidMetric {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
