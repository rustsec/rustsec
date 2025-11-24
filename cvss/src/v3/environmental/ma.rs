//! CVSS v3.1 Environmental Metric Group - Modified Availability (MA)

use crate::v3::base::Availability;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Availability (MA) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedAvailability(pub Option<Availability>);

impl From<Availability> for ModifiedAvailability {
    fn from(a: Availability) -> Self {
        ModifiedAvailability(Some(a))
    }
}

impl Metric for ModifiedAvailability {
    const TYPE: MetricType = MetricType::MA;

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

impl fmt::Display for ModifiedAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedAvailability {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedAvailability(None))
        } else {
            Ok(ModifiedAvailability(Some(s.parse()?)))
        }
    }
}
