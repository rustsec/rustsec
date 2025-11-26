//! CVSS v3.1 Environmental Metric Group - Modified Availability (MA)

use crate::v3::base::Availability;
use crate::v3::metric::ModifiedMetric;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Availability (MA) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedAvailability {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [Availability])
    Modified(Availability),
}

impl Metric for ModifiedAvailability {
    const TYPE: MetricType = MetricType::MA;

    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            ModifiedAvailability::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }
}

impl ModifiedMetric<Availability> for ModifiedAvailability {
    fn modified_score(self, base: Option<Availability>) -> f64 {
        match self {
            ModifiedAvailability::Modified(v) => v.score(),
            ModifiedAvailability::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
        }
    }
}

impl fmt::Display for ModifiedAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedAvailability {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAvailability::NotDefined)
        } else {
            Ok(ModifiedAvailability::Modified(s.parse()?))
        }
    }
}
