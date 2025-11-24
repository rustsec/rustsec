//! CVSS v3.1 Environmental Metric Group - Modified Confidentiality (MC)

use crate::v3::base::Confidentiality;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Confidentiality (MC) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedConfidentiality {
    pub modified: Option<Confidentiality>,
    pub base: Option<Confidentiality>,
}

impl ModifiedConfidentiality {
    pub fn from_str(s: &str, base: Option<Confidentiality>) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedConfidentiality { modified: None, base })
        } else {
            Ok(ModifiedConfidentiality {
                modified: Some(s.parse()?),
                base,
            })
        }
    }
}

impl Metric for ModifiedConfidentiality {
    const TYPE: MetricType = MetricType::MC;

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

impl fmt::Display for ModifiedConfidentiality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}
