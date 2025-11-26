//! CVSS v3.1 Environmental Metric Group - Modified User Interaction (MUI)

use crate::v3::base::UserInteraction;
use crate::v3::metric::ModifiedMetric;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified User Interaction (MUI) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedUserInteraction {
    /// Not Defined (X)
    NotDefined,

    /// Modified (see [UserInteraction])
    Modified(UserInteraction),
}

impl Metric for ModifiedUserInteraction {
    const TYPE: MetricType = MetricType::MUI;

    fn score(self) -> f64 {
        0.0
    }

    fn as_str(self) -> &'static str {
        match self {
            ModifiedUserInteraction::Modified(v) => v.as_str(),
            Self::NotDefined => "X",
        }
    }
}

impl ModifiedMetric<UserInteraction> for ModifiedUserInteraction {
    fn modified_score(self, base: Option<UserInteraction>) -> f64 {
        match self {
            ModifiedUserInteraction::Modified(v) => v.score(),
            ModifiedUserInteraction::NotDefined => base.map(|v| v.score()).unwrap_or(0.0),
        }
    }
}

impl fmt::Display for ModifiedUserInteraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedUserInteraction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedUserInteraction::NotDefined)
        } else {
            Ok(ModifiedUserInteraction::Modified(s.parse()?))
        }
    }
}
