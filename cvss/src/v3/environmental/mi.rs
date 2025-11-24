//! CVSS v3.1 Environmental Metric Group - Modified Integrity (MI)

use crate::v3::base::Integrity;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Integrity (MI) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedIntegrity(pub Option<Integrity>);

impl From<Integrity> for ModifiedIntegrity {
    fn from(i: Integrity) -> Self {
        ModifiedIntegrity(Some(i))
    }
}

impl Metric for ModifiedIntegrity {
    const TYPE: MetricType = MetricType::MI;

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

impl fmt::Display for ModifiedIntegrity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedIntegrity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedIntegrity(None))
        } else {
            Ok(ModifiedIntegrity(Some(s.parse()?)))
        }
    }
}
