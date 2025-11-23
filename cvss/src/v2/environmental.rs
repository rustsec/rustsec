//! CVSS 2.0 Environmental Metric Group

mod ar;
mod cdp;
mod cr;
mod ir;
mod td;

use core::cmp::min;

#[cfg(feature = "std")]
use crate::v2::Score;
use crate::v2::{Metric, Vector};

pub use self::{
    ar::AvailabilityRequirement, cdp::CollateralDamagePotential, cr::ConfidentialityRequirement,
    ir::IntegrityRequirement, td::TargetDistribution,
};

impl Vector {
    /// Calculate Environmental CVSS score
    ///
    /// Described in CVSS v2.0 Specification: Section 3.2.3:
    /// <https://www.first.org/cvss/v2/guide#3-2-3-Environmental-Equation>
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn environmental_score(&self) -> Score {
        let adjusted_temporal = self.temporal_score_internal(self.adjusted_impact());
        let cdp_score = self.cdp.map(|cdp| cdp.score()).unwrap_or(0.0);
        let td_score = self.td.map(|td| td.score()).unwrap_or(1.0);
        let score = (adjusted_temporal.value() + (10.0 - adjusted_temporal.value()) * cdp_score) * td_score;

        Score::new(score).roundup()
    }

    /// Calculate Adjusted Impact: modified impact score based on
    /// environmental requirements.
    pub fn adjusted_impact(&self) -> Score {
        let c_score = self.c.map(|c| c.score()).unwrap_or(0.0);
        let i_score = self.i.map(|i| i.score()).unwrap_or(0.0);
        let a_score = self.a.map(|a| a.score()).unwrap_or(0.0);

        let cr_score = self.cr.map(|cr| cr.score()).unwrap_or(1.0);
        let ir_score = self.ir.map(|ir| ir.score()).unwrap_or(1.0);
        let ar_score = self.ar.map(|ar| ar.score()).unwrap_or(1.0);

        (10.0_f64.min(10.41 * (1.0 - (1.0 - c_score * cr_score) * (1.0 - i_score * ir_score) * (1.0 - a_score * ar_score)))).into()
    }
}
