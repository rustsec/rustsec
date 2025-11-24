//! CVSS v3.1 Environmental Metric Group - Modified Attack Vector (MAV)

use crate::v3::base::AttackVector;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Attack Vector (MAV) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedAttackVector(pub Option<AttackVector>);

impl From<AttackVector> for ModifiedAttackVector {
    fn from(av: AttackVector) -> Self {
        ModifiedAttackVector(Some(av))
    }
}

impl Metric for ModifiedAttackVector {
    const TYPE: MetricType = MetricType::MAV;

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

impl fmt::Display for ModifiedAttackVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedAttackVector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAttackVector(None))
        } else {
            Ok(ModifiedAttackVector(Some(s.parse()?)))
        }
    }
}
