//! CVSS v3.1 Environmental Metric Group - Modified User Interaction (MUI)

use crate::v3::base::UserInteraction;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified User Interaction (MUI) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedUserInteraction(pub Option<UserInteraction>);

impl From<UserInteraction> for ModifiedUserInteraction {
    fn from(ui: UserInteraction) -> Self {
        ModifiedUserInteraction(Some(ui))
    }
}

impl Metric for ModifiedUserInteraction {
    const TYPE: MetricType = MetricType::MUI;

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

impl fmt::Display for ModifiedUserInteraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedUserInteraction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedUserInteraction(None))
        } else {
            Ok(ModifiedUserInteraction(Some(s.parse()?)))
        }
    }
}
