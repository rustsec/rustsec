//! CVSS v3.1 Environmental Metric Group - Modified Integrity (MI)

use crate::v3::base::Integrity;
use crate::v3::metric::ModifiedMetric;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Integrity (MI) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedIntegrity {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [Integrity])
    Modified(Integrity),
}

impl Metric for ModifiedIntegrity {
    const TYPE: MetricType = MetricType::MI;

    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            ModifiedIntegrity::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }
}

impl ModifiedMetric<Integrity> for ModifiedIntegrity {
    fn modified_score(self, base: Option<Integrity>) -> f64 {
        match self {
            ModifiedIntegrity::Modified(v) => v.score(),
            ModifiedIntegrity::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
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
            Ok(ModifiedIntegrity::NotDefined)
        } else {
            Ok(ModifiedIntegrity::Modified(s.parse()?))
        }
    }
}
