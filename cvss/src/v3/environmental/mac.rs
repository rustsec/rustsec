//! CVSS v3.1 Environmental Metric Group - Modified Attack Complexity (MAC)

use crate::v3::base::AttackComplexity;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Attack Complexity (MAC) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedAttackComplexity(pub Option<AttackComplexity>);

impl From<AttackComplexity> for ModifiedAttackComplexity {
    fn from(ac: AttackComplexity) -> Self {
        ModifiedAttackComplexity(Some(ac))
    }
}

impl Metric for ModifiedAttackComplexity {
    const TYPE: MetricType = MetricType::MAC;

    fn score(self) -> f64 {
        self.0.map_or(0.0, |v| v.score())
    }

    fn as_str(self) -> &'static str {
        match self.0 {
            Some(v) => v.as_str(),
            None => "X",
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
            Ok(ModifiedAttackComplexity(None))
        } else {
            Ok(ModifiedAttackComplexity(Some(s.parse()?)))
        }
    }
}
