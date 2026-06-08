//! CVSS v3.1 Environmental Metric Group - Modified Confidentiality (MC)

use crate::v3::base::Confidentiality;
use crate::v3::metric::ModifiedMetric;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Confidentiality (MC) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedConfidentiality {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [Confidentiality])
    Modified(Confidentiality),
}

impl Metric for ModifiedConfidentiality {
    const TYPE: MetricType = MetricType::MC;

    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            ModifiedConfidentiality::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }
}

impl ModifiedMetric<Confidentiality> for ModifiedConfidentiality {
    fn modified_score(self, base: Option<Confidentiality>) -> f64 {
        match self {
            ModifiedConfidentiality::Modified(v) => v.score(),
            ModifiedConfidentiality::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
        }
    }
}

impl fmt::Display for ModifiedConfidentiality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedConfidentiality {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedConfidentiality::NotDefined)
        } else {
            Ok(ModifiedConfidentiality::Modified(s.parse()?))
        }
    }
}
