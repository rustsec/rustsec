//! CVSS v3.1 Environmental Metric Group - Modified Attack Vector (MAV)

use crate::v3::base::AttackVector;
use crate::v3::metric::ModifiedMetric;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Attack Vector (MAV) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedAttackVector {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [AttackVector])
    Modified(AttackVector),
}

impl Metric for ModifiedAttackVector {
    const TYPE: MetricType = MetricType::MAV;

    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            ModifiedAttackVector::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }
}

impl ModifiedMetric<AttackVector> for ModifiedAttackVector {
    fn modified_score(self, base: Option<AttackVector>) -> f64 {
        match self {
            ModifiedAttackVector::Modified(v) => v.score(),
            ModifiedAttackVector::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
        }
    }
}

impl fmt::Display for ModifiedAttackVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedAttackVector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAttackVector::NotDefined)
        } else {
            Ok(ModifiedAttackVector::Modified(s.parse()?))
        }
    }
}
