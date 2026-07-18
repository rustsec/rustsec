//! CVSS v3.1 Environmental Metric Group - Modified Privileges Required (MPR)

use core::{fmt, str::FromStr};

use crate::Error;
use crate::v3::{
    Metric, MetricType,
    metric::{ModifiedMetric, base::PrivilegesRequired},
};

/// Modified Privileges Required (MPR) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedPrivilegesRequired {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [PrivilegesRequired])
    Modified(PrivilegesRequired),
}

impl ModifiedPrivilegesRequired {
    /// Calculate the Scoped Score for the Modified Privileges Required (MPR)
    /// metric
    ///
    /// Its value depends on whether the scope of the
    /// [crate::v3::metric::environmental::ModifiedScope] (or
    /// [crate::v3::metric::base::Scope] base) metric has changed.
    pub fn scoped_score(self, scope_changed: bool, base: Option<PrivilegesRequired>) -> f64 {
        match self {
            Self::Modified(v) => v.scoped_score(scope_changed),
            Self::NotDefined => base.map(|b| b.scoped_score(scope_changed)).unwrap_or(0.0),
        }
    }
}

impl Metric for ModifiedPrivilegesRequired {
    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }

    const TYPE: MetricType = MetricType::MPR;
}

impl ModifiedMetric<PrivilegesRequired> for ModifiedPrivilegesRequired {
    fn modified_score(self, base: Option<PrivilegesRequired>) -> f64 {
        match self {
            Self::Modified(v) => v.score(),
            Self::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
        }
    }
}

impl fmt::Display for ModifiedPrivilegesRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedPrivilegesRequired {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(Self::NotDefined)
        } else {
            Ok(Self::Modified(s.parse()?))
        }
    }
}
