//! CVSS v3.1 Environmental Metric Group - Modified Attack Complexity (MAC)

use crate::v3::base::AttackComplexity;
use crate::{Error, Metric, MetricType};
use core::fmt;
use alloc::string::String;

/// Modified Attack Complexity (MAC) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedAttackComplexity {
    pub modified: Option<AttackComplexity>,
    pub base: Option<AttackComplexity>,
}

impl ModifiedAttackComplexity {
    pub fn from_str(s: &str, base: Option<AttackComplexity>) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAttackComplexity {
                modified: None,
                base,
            })
        } else {
            Ok(ModifiedAttackComplexity {
                modified: Some(s.parse()?),
                base,
            })
        }
    }
}

impl Metric for ModifiedAttackComplexity {
    const TYPE: MetricType = MetricType::MAC;

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

impl fmt::Display for ModifiedAttackComplexity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}
