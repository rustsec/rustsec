//! CVSS v3.1 Environmental Metric Group - Modified Attack Complexity (MAC)

use crate::v3::base::AttackComplexity;
use crate::v3::metric::ModifiedMetric;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Attack Complexity (MAC) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedAttackComplexity {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [AttackComplexity])
    Modified(AttackComplexity),
}

impl Metric for ModifiedAttackComplexity {
    const TYPE: MetricType = MetricType::MAC;

    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            ModifiedAttackComplexity::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }
}

impl ModifiedMetric<AttackComplexity> for ModifiedAttackComplexity {
    fn modified_score(self, base: Option<AttackComplexity>) -> f64 {
        match self {
            ModifiedAttackComplexity::Modified(v) => v.score(),
            ModifiedAttackComplexity::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
        }
    }
}

impl fmt::Display for ModifiedAttackComplexity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedAttackComplexity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAttackComplexity::NotDefined)
        } else {
            Ok(ModifiedAttackComplexity::Modified(s.parse()?))
        }
    }
}
