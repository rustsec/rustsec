//! CVSS v2.0 scores

use crate::severity::Severity;

/// CVSS V2.0 scores.
///
/// Formula described in CVSS v2.0 Specification: Section 3.2:
/// <https://www.first.org/cvss/v2/guide#3-2-Equations>
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Score(f64);

impl Score {
    /// Create a new score object
    pub fn new(score: f64) -> Score {
        Score(score)
    }

    /// Get the score as a floating point value
    pub fn value(self) -> f64 {
        self.0
    }

    /// Round the score up to 1 decimal (`round_to_1_decimal`)
    #[cfg(feature = "std")]
    pub fn roundup(self) -> Score {
        let rounded = (self.0 * 10.0).round() / 10.0;
        Score(rounded)
    }

    /// Convert the numeric score into a `Severity`
    pub fn severity(self) -> Severity {
        if self.0 < 0.1 {
            Severity::None
        } else if self.0 < 4.0 {
            Severity::Low
        } else if self.0 < 7.0 {
            Severity::Medium
        } else if self.0 < 9.0 {
            Severity::High
        } else {
            Severity::Critical
        }
    }
}

impl From<f64> for Score {
    fn from(score: f64) -> Score {
        Score(score)
    }
}

impl From<Score> for f64 {
    fn from(score: Score) -> f64 {
        score.value()
    }
}

impl From<Score> for Severity {
    fn from(score: Score) -> Severity {
        score.severity()
    }
}
