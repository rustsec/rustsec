//! CVSS v2.0 Base Metric Group - Integrity Impact (C)

use crate::Error;
use crate::v2::{Metric, MetricType};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Integrity Impact (C) - CVSS v2.0 Base Metric Group
///
/// Described in CVSS v2.0 Specification: Section 2.1.5:
/// <https://www.first.org/cvss/v2/guide#2-1-5-Integrity-Impact-I>
///
/// > This metric measures the impact to integrity of a successfully exploited
/// > vulnerability. Integrity refers to the trustworthiness and guaranteed
/// > veracity of information. The possible values for this metric are listed in
/// > Table 5. Increased integrity impact increases the vulnerability score.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum IntegrityImpact {
    None,
    Partial,
    Complete,
}

impl Metric for IntegrityImpact {
    const TYPE: MetricType = MetricType::I;

    fn score(self) -> f64 {
        match self {
            IntegrityImpact::None => 0.0,
            IntegrityImpact::Partial => 0.275,
            IntegrityImpact::Complete => 0.660,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            IntegrityImpact::None => "N",
            IntegrityImpact::Partial => "P",
            IntegrityImpact::Complete => "C",
        }
    }
}

impl fmt::Display for IntegrityImpact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for IntegrityImpact {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "N" => Ok(IntegrityImpact::None),
            "P" => Ok(IntegrityImpact::Partial),
            "C" => Ok(IntegrityImpact::Complete),
            _ => Err(Error::InvalidV2Metric {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
