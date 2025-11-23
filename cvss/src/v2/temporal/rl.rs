//! CVSS v2.0 Temporal Metric - Remediation Level (RL)

use crate::Error;
use crate::v2::{Metric, MetricType};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Remediation Level (RL) - CVSS v2.0 Temporal Metric Group
///
/// Described in CVSS v2.0 Specification: Section 2.2.2:
/// <https://www.first.org/cvss/v2/guide#2-2-2-Remediation-Level-RL>
///
/// > The remediation level of a vulnerability is an important factor for
/// prioritization. The typical vulnerability is unpatched when initially
/// published.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RemediationLevel {
    /// Official Fix (OF)
    /// > A complete vendor solution is available. Either the vendor has issued
    /// > an official patch, or an upgrade is available.
    OfficialFix,

    /// Temporary Fix (TF)
    /// > There is an official but temporary fix available. This includes
    /// > instances where the vendor issues a temporary hotfix, tool, or
    /// > workaround.
    TemporaryFix,

    /// Workaround (W)
    /// > There is an unofficial, non-vendor solution available. In some cases,
    /// > users of the affected technology will create a patch of their own or
    /// > provide steps to work around or otherwise mitigate the vulnerability.
    Workaround,

    //// Unavailable (U)
    /// > There is either no solution available or it is impossible to apply.
    Unavailable,

    /// Not Defined (ND)
    /// > Assigning this value to the metric will not influence the score. It is
    /// > a signal to the equation to skip this metric.
    NotDefined,
}

impl Metric for RemediationLevel {
    const TYPE: MetricType = MetricType::RL;
    fn score(self) -> f64 {
        match self {
            RemediationLevel::OfficialFix => 0.87,
            RemediationLevel::TemporaryFix => 0.90,
            RemediationLevel::Workaround => 0.95,
            RemediationLevel::Unavailable => 1.0,
            RemediationLevel::NotDefined => 1.0,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            RemediationLevel::OfficialFix => "OF",
            RemediationLevel::TemporaryFix => "TF",
            RemediationLevel::Workaround => "W",
            RemediationLevel::Unavailable => "U",
            RemediationLevel::NotDefined => "ND",
        }
    }
}

impl fmt::Display for RemediationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for RemediationLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "OF" => Ok(RemediationLevel::OfficialFix),
            "TF" => Ok(RemediationLevel::TemporaryFix),
            "W" => Ok(RemediationLevel::Workaround),
            "U" => Ok(RemediationLevel::Unavailable),
            "ND" => Ok(RemediationLevel::NotDefined),
            _ => Err(Error::InvalidMetricV2 {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
