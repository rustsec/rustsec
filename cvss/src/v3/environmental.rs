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

use crate::v3::{Score, Vector};

impl Vector {
    /// Calculate the CVSS 3.1 Environmental Score for this vector.
    ///
    /// Described in CVSS v3.1 Specification: Section 7.3:
    /// <https://www.first.org/cvss/v3-1/specification-document#7-3-Environmental-Metrics-Equations>
    pub fn environmental_score(&self) -> Score {
        let environmental_score = 0.0;

        Score::new(environmental_score).roundup()
    }
}
