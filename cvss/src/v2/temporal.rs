//! CVSS 2.0 Temporal Metric Group

mod e;
mod rc;
mod rl;

pub use self::{e::Exploitability, rc::ReportConfidence, rl::RemediationLevel};
use super::Score;
use crate::v2::{Metric, Vector};

impl Vector {
    /// Calculate Temporal CVSS score
    ///
    /// Described in CVSS v2.0 Specification: Section 3.2.2:
    /// <https://www.first.org/cvss/v2/guide#3-2-2-Temporal-Equation>
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn temporal_score(&self) -> Score {
        let base_score = self.base_score().value();
        let e_score = self.e.map(|e| e.score()).unwrap_or(1.0);
        let rl_score = self.rl.map(|rl| rl.score()).unwrap_or(1.0);
        let rc_score = self.rc.map(|rc| rc.score()).unwrap_or(1.0);

        let score = base_score * e_score * rl_score * rc_score;

        Score::new(score).roundup()
    }
}
