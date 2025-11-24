//! CVSS v3.1 Environmental Metric Group - Modified Scope (MS)

use crate::v3::base::Scope;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Scope (MS) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedScope(pub Option<Scope>);

impl From<Scope> for ModifiedScope {
    fn from(s: Scope) -> Self {
        ModifiedScope(Some(s))
    }
}

impl Metric for ModifiedScope {
    const TYPE: MetricType = MetricType::MS;

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

impl fmt::Display for ModifiedScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedScope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedScope(None))
        } else {
            Ok(ModifiedScope(Some(s.parse()?)))
        }
    }
}
