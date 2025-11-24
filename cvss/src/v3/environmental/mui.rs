//! CVSS v3.1 Environmental Metric Group - Modified User Interaction (MUI)

use crate::v3::base::UserInteraction;
use crate::{Error, Metric, MetricType};
use core::{fmt, str::FromStr};

/// Modified User Interaction (MUI) - CVSS v3.1 Environmental Metric Group
///
/// Described in CVSS v3.1 Specification: Section 4.2:
/// <https://www.first.org/cvss/v3-1/specification-document#4-2-Modified-Base-Metrics>
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ModifiedUserInteraction {
    pub modified: Option<UserInteraction>,
    pub base: Option<UserInteraction>,
}

impl ModifiedUserInteraction {
    pub fn from_str(s: &str, base: Option<UserInteraction>) -> Result<Self, Error> {
        if s == "X" {
            Ok(ModifiedUserInteraction { modified: None, base })
        } else {
            Ok(ModifiedUserInteraction {
                modified: Some(s.parse()?),
                base,
            })
        }
    }
}

impl Metric for ModifiedUserInteraction {
    const TYPE: MetricType = MetricType::MUI;

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

impl fmt::Display for ModifiedUserInteraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}
