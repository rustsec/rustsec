//! Remediation Level (RL)

use crate::{Error, Metric, MetricType, Result};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Remediation Level (RL) - CVSS v3.1 Temporal Metric Group
/// > The Remediation Level of a vulnerability is an important factor for
/// > prioritization. The typical vulnerability is unpatched when initially
/// > published.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RemediationLevel {
    /// Not Defined (X)
    /// > Assigning this value indicates there is insufficient information to
    /// > choose one of the other values, and has no impact on the overall
    /// > Temporal Score, i.e., it has the same effect on scoring as assigning
    /// > Unavailable.
    NotDefined,

    /// Unavailable (U)
    /// > There is either no solution available or it is impossible to apply.
    Unavailable,

    /// Workaround (W)
    /// > There is an unofficial, non-vendor solution available. In some cases,
    /// > users of the affected technology will create a patch of their own or
    /// > provide steps to work around or otherwise mitigate the vulnerability.
    Workaround,

    /// Temporary Fix (T)
    /// > There is an official but temporary fix available. This includes
    /// > instances where the vendor issues a temporary hotfix, tool, or
    /// > workaround.
    TemporaryFix,

    /// Official Fix (O)
    /// > A complete vendor solution is available. Either the vendor has issued
    /// > an official patch, or an upgrade is available.
    OfficialFix,
}

impl Metric for RemediationLevel {
    const TYPE: MetricType = MetricType::RL;

    fn score(self) -> f64 {
        match self {
            RemediationLevel::NotDefined => 1.0,
            RemediationLevel::Unavailable => 1.0,
            RemediationLevel::Workaround => 0.97,
            RemediationLevel::TemporaryFix => 0.96,
            RemediationLevel::OfficialFix => 0.95,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            RemediationLevel::NotDefined => "X",
            RemediationLevel::Unavailable => "U",
            RemediationLevel::Workaround => "W",
            RemediationLevel::TemporaryFix => "T",
            RemediationLevel::OfficialFix => "O",
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

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(RemediationLevel::NotDefined),
            "U" => Ok(RemediationLevel::Unavailable),
            "W" => Ok(RemediationLevel::Workaround),
            "T" => Ok(RemediationLevel::TemporaryFix),
            "O" => Ok(RemediationLevel::OfficialFix),
            _ => Err(Error::InvalidMetric {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
