//! CVSS v3.1 Environmental Metric Group - Modified Confidentiality (MC)

use core::{fmt, str::FromStr};

use crate::Error;
use crate::v3::{
    Metric, MetricType,
    metric::{ModifiedMetric, base::Confidentiality},
};

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
    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }

    const TYPE: MetricType = MetricType::MC;
}

impl ModifiedMetric<Confidentiality> for ModifiedConfidentiality {
    fn modified_score(self, base: Option<Confidentiality>) -> f64 {
        match self {
            Self::Modified(v) => v.score(),
            Self::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
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
            Ok(Self::NotDefined)
        } else {
            Ok(Self::Modified(s.parse()?))
        }
    }
}
