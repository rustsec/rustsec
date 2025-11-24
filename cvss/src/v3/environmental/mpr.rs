//! CVSS v3.1 Environmental Metric Group - Modified Privileges Required (MPR)

use crate::v3::base::PrivilegesRequired;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified Privileges Required (MPR) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedPrivilegesRequired(pub Option<PrivilegesRequired>);

impl From<PrivilegesRequired> for ModifiedPrivilegesRequired {
    fn from(pr: PrivilegesRequired) -> Self {
        ModifiedPrivilegesRequired(Some(pr))
    }
}

impl Metric for ModifiedPrivilegesRequired {
    const TYPE: MetricType = MetricType::MPR;

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

impl fmt::Display for ModifiedPrivilegesRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedPrivilegesRequired {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedPrivilegesRequired(None))
        } else {
            Ok(ModifiedPrivilegesRequired(Some(s.parse()?)))
        }
    }
}
