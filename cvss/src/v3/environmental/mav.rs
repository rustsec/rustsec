//! CVSS v3.1 Environmental Metric Group - Modified Attack Vector (MAV)

use crate::v3::base::AttackVector;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Attack Vector (MAV) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedAttackVector {
    pub modified: Option<AttackVector>,
    pub base: Option<AttackVector>,
}

impl ModifiedAttackVector {
    pub fn from_str(s: &str, base: Option<AttackVector>) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAttackVector { modified: None, base })
        } else {
            Ok(ModifiedAttackVector {
                modified: Some(s.parse()?),
                base,
            })
        }
    }
}

impl Metric for ModifiedAttackVector {
    const TYPE: MetricType = MetricType::MAV;

    fn score(self) -> f64 {
        if let Some(m) = self.modified {
            m.score()
        } else if let Some(b) = self.base {
            b.score()
        } else {
            0.0
        }
    }

    fn as_str(self) -> &'static str {
        match self.modified {
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
