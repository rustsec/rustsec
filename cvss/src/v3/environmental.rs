//! CVSS v3.1 Environmental Metric Group

mod ar;
mod cr;
mod ir;
mod ma;
mod mac;
mod mav;
mod mc;
mod mi;
mod mpr;
mod ms;
mod mui;

pub use self::{
    ar::AvailabilityRequirement, cr::ConfidentialityRequirement, ir::IntegrityRequirement,
    ma::ModifiedAvailability, mac::ModifiedAttackComplexity, mav::ModifiedAttackVector,
    mc::ModifiedConfidentiality, mi::ModifiedIntegrity, mpr::ModifiedPrivilegesRequired,
    ms::ModifiedScope, mui::ModifiedUserInteraction,
};

use crate::v3::base::Scope;
use crate::v3::metric::{Metric, ModifiedMetric};
use crate::v3::temporal::{ExploitCodeMaturity, RemediationLevel, ReportConfidence};
use crate::v3::{Score, Vector};

impl Vector {
    /// Calculate the CVSS 3.1 Environmental Score for this vector.
    ///
    /// Described in CVSS v3.1 Specification: Section 7.3:
    /// <https://www.first.org/cvss/v3-1/specification-document#7-3-Environmental-Metrics-Equations>
    pub fn environmental_score(&self) -> Score {
        let modified_impact = self.modified_impact();
        let modified_exploitability = self.modified_exploitability();
        let e_score = self.e.unwrap_or(ExploitCodeMaturity::NotDefined).score();
        let rl_score = self.rl.unwrap_or(RemediationLevel::NotDefined).score();
        let rc_score = self.rc.unwrap_or(ReportConfidence::NotDefined).score();

        let environmental_score = if modified_impact <= 0.0 {
            0.0
        } else if !self.is_modified_scope_changed() {
            Score::new((modified_impact + modified_exploitability).min(10.0))
                .roundup_for_version(self.minor_version)
                .value()
                * e_score
                * rl_score
                * rc_score
        } else {
            Score::new((1.08 * (modified_impact + modified_exploitability)).min(10.0))
                .roundup_for_version(self.minor_version)
                .value()
                * e_score
                * rl_score
                * rc_score
        };
        Score::new(environmental_score).roundup_for_version(self.minor_version)
    }

    /// Calculate the modified impact sub-score (MISS) for environmental score
    /// calculations.    
    pub(crate) fn modified_impact_sub_score(&self) -> f64 {
        let cr_score = self
            .cr
            .unwrap_or(ConfidentialityRequirement::NotDefined)
            .score();
        let ir_score = self.ir.unwrap_or(IntegrityRequirement::NotDefined).score();
        let ar_score = self
            .ar
            .unwrap_or(AvailabilityRequirement::NotDefined)
            .score();
        let c_score = self
            .mc
            .unwrap_or(ModifiedConfidentiality::NotDefined)
            .modified_score(self.c);
        let i_score = self
            .mi
            .unwrap_or(ModifiedIntegrity::NotDefined)
            .modified_score(self.i);
        let a_score = self
            .ma
            .unwrap_or(ModifiedAvailability::NotDefined)
            .modified_score(self.a);

        let miss = 1.0
            - (1.0 - cr_score * c_score) * (1.0 - ir_score * i_score) * (1.0 - ar_score * a_score);
        miss.min(0.915)
    }

    pub(crate) fn modified_impact(&self) -> f64 {
        let miss = self.modified_impact_sub_score();
        if self.is_modified_scope_changed() {
            7.52 * (miss - 0.029) - 3.25 * (miss * 0.9731 - 0.02).powf(13.0)
        } else {
            6.42 * miss
        }
    }

    pub(crate) fn modified_exploitability(&self) -> f64 {
        let av_score = self
            .mav
            .unwrap_or(ModifiedAttackVector::NotDefined)
            .modified_score(self.av);
        let ac_score = self
            .mac
            .unwrap_or(ModifiedAttackComplexity::NotDefined)
            .modified_score(self.ac);
        let pr_score = self
            .mpr
            .unwrap_or(ModifiedPrivilegesRequired::NotDefined)
            .scoped_score(self.is_modified_scope_changed(), self.pr);
        let ui_score = self
            .mui
            .unwrap_or(ModifiedUserInteraction::NotDefined)
            .modified_score(self.ui);

        8.22 * av_score * ac_score * pr_score * ui_score
    }

    pub(crate) fn is_modified_scope_changed(&self) -> bool {
        match self.ms {
            Some(modified_scope) => match modified_scope {
                ModifiedScope::Modified(scope) => scope == Scope::Changed,
                ModifiedScope::NotDefined => self.is_scope_changed(),
            },
            None => self.is_scope_changed(),
        }
    }
}
