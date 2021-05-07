//! Qualitative Severity Rating Scale

use crate::error::{Error, ErrorKind};
#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Qualitative Severity Rating Scale
///
/// Described in CVSS v3.1 Specification: Section 5:
/// <https://www.first.org/cvss/specification-document#t17>
///
/// > For some purposes it is useful to have a textual representation of the
/// > numeric Base, Temporal and Environmental scores.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Severity {
    /// None: CVSS Score 0.0
    None,

    /// Low: CVSS Score 0.1 - 3.9
    Low,

    /// Medium: CVSS Score 4.0 - 6.9
    Medium,

    /// High: CVSS Score 7.0 - 8.9
    High,

    /// Critical: CVSS Score 9.0 - 10.0
    Critical,
}

impl Severity {
    /// Get a `str` describing the severity level
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::None => "none",
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        }
    }
}

impl FromStr for Severity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "none" => Severity::None,
            "low" => Severity::Low,
            "medium" => Severity::Medium,
            "high" => Severity::High,
            "critical" => Severity::Critical,
            _ => fail!(
                ErrorKind::Parse,
                "invalid CVSS Qualitative Severity Rating Scale value: {}",
                s
            ),
        })
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Severity {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
