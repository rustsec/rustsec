//! CVSS v3.1 Environmental Metric Group - Modified Scope (MS)

use crate::v3::base::Scope;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Scope (MS) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedScope {
    pub modified: Option<Scope>,
    pub base: Option<Scope>,
}

impl ModifiedScope {
    pub fn from_str(s: &str, base: Option<Scope>) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedScope { modified: None, base })
        } else {
            Ok(ModifiedScope {
                modified: Some(s.parse()?),
                base,
            })
        }
    }
}

impl Metric for ModifiedScope {
    const TYPE: MetricType = MetricType::MS;

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

impl fmt::Display for ModifiedScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}
