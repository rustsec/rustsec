//! CVSS 2.0 Base Metric Group

mod a;
mod ac;
mod au;
mod av;
mod c;
mod i;

pub use self::{
    a::AvailabilityImpact, ac::AccessComplexity, au::Authentication, av::AccessVector,
    c::ConfidentialityImpact, i::IntegrityImpact,
};
use super::Score;
use crate::v2::{Metric, Vector};

impl Vector {
    /// Calculate Base CVSS score: overall value for determining the severity
    /// of a vulnerability, generally referred to as the "CVSS score".
    ///
    /// Described in CVSS v2.0 Specification: Section 3.2.1:
    /// <https://www.first.org/cvss/v2/guide#3-2-1-Base-Equation>
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn base_score(&self) -> Score {
        let exploitability = self.exploitability().value();
        let impact = self.impact().value();

        let f_impact = if impact == 0.0 { 0.0 } else { 1.176 };

        let score = ((0.6 * impact) + (0.4 * exploitability) - 1.5) * f_impact;

        Score::new(score).roundup()
    }

    /// Calculate Base Exploitability score: sub-score for measuring
    /// ease of exploitation.
    ///
    /// Described in CVSS v2.0 Specification: Section 3.2.1:
    /// <https://www.first.org/cvss/v2/guide#3-2-1-Base-Equation>
    pub fn exploitability(&self) -> Score {
        let av_score = self.av.map(|av| av.score()).unwrap_or(0.0);
        let ac_score = self.ac.map(|ac| ac.score()).unwrap_or(0.0);
        let au_score = self.au.map(|au| au.score()).unwrap_or(0.0);

        (20.0 * av_score * ac_score * au_score).into()
    }

    /// Calculate Base Impact Score: sub-score for measuring the
    /// consequences of successful exploitation.
    ///
    /// Described in CVSS v2.0 Specification: Section 3.2.1:
    /// <https://www.first.org/cvss/v2/guide#3-2-1-Base-Equation>
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn impact(&self) -> Score {
        let c_score = self.c.map(|c| c.score()).unwrap_or(0.0);
        let i_score = self.i.map(|i| i.score()).unwrap_or(0.0);
        let a_score = self.a.map(|a| a.score()).unwrap_or(0.0);
        (10.41 * (1.0 - ((1.0 - c_score) * (1.0 - i_score) * (1.0 - a_score)))).into()
    }
}
