//! CVSS v3.1 Environmental Metric Group - Modified Availability (MA)

use crate::v3::base::Availability;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Availability (MA) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedAvailability {
    pub modified: Option<Availability>,
    pub base: Option<Availability>,
}

impl ModifiedAvailability {
    pub fn from_str(s: &str, base: Option<Availability>) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAvailability { modified: None, base })
        } else {
            Ok(ModifiedAvailability {
                modified: Some(s.parse()?),
                base,
            })
        }
    }
}

impl Metric for ModifiedAvailability {
    const TYPE: MetricType = MetricType::MA;

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

impl fmt::Display for ModifiedAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}
