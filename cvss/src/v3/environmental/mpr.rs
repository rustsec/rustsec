//! CVSS v3.1 Environmental Metric Group - Modified Privileges Required (MPR)

use crate::v3::base::PrivilegesRequired;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Privileges Required (MPR) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedPrivilegesRequired {
    pub modified: Option<PrivilegesRequired>,
    pub base: Option<PrivilegesRequired>,
}

impl ModifiedPrivilegesRequired {
    pub fn from_str(s: &str, base: Option<PrivilegesRequired>) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedPrivilegesRequired {
                modified: None,
                base,
            })
        } else {
            Ok(ModifiedPrivilegesRequired {
                modified: Some(s.parse()?),
                base,
            })
        }
    }

    pub fn scoped_score(self, scope_changed: bool) -> f64 {
        if let Some(m) = self.modified {
            m.scoped_score(scope_changed)
        } else if let Some(b) = self.base {
            b.scoped_score(scope_changed)
        } else {
            0.0
        }
    }
}

impl Metric for ModifiedPrivilegesRequired {
    const TYPE: MetricType = MetricType::MPR;

    fn score(self) -> f64 {
        // Default to unscoped score (false) for Metric::score
        self.scoped_score(false)
    }

    fn as_str(self) -> &'static str {
        match self.modified {
            Some(v) => v.as_str(),
            None => "X",
        }
    }
}

impl fmt::Display for ModifiedPrivilegesRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}
