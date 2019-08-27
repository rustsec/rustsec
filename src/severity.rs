//! Qualitative Severity Rating Scale

use std::fmt;

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

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
