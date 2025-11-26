//! CVSS v3.1 Environmental Metric Group - Modified Scope (MS)

use crate::v3::base::Scope;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Scope (MS) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedScope {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [Scope])
    Modified(Scope),
}

impl Metric for ModifiedScope {
    const TYPE: MetricType = MetricType::MS;

    fn score(self) -> f64 {
        match self {
            ModifiedScope::Modified(v) => v.score(),
            ModifiedScope::NotDefined => 0.0,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            ModifiedScope::Modified(v) => v.as_str(),
            ModifiedScope::NotDefined => "X",
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
            Ok(ModifiedScope::NotDefined)
        } else {
            Ok(ModifiedScope::Modified(s.parse()?))
        }
    }
}
