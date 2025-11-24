//! CVSS v3.1 Environmental Metric Group - Modified Confidentiality (MC)

use crate::v3::base::Confidentiality;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Confidentiality (MC) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedConfidentiality(pub Option<Confidentiality>);

impl From<Confidentiality> for ModifiedConfidentiality {
    fn from(c: Confidentiality) -> Self {
        ModifiedConfidentiality(Some(c))
    }
}

impl Metric for ModifiedConfidentiality {
    const TYPE: MetricType = MetricType::MC;

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

impl fmt::Display for ModifiedConfidentiality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedConfidentiality {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedConfidentiality(None))
        } else {
            Ok(ModifiedConfidentiality(Some(s.parse()?)))
        }
    }
}
