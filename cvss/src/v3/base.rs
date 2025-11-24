//! CVSS v3.1 Base Metric Group

mod a;
mod ac;
mod av;
mod c;
mod i;
mod pr;
mod s;
mod ui;

pub use self::{
    a::Availability, ac::AttackComplexity, av::AttackVector, c::Confidentiality, i::Integrity,
    pr::PrivilegesRequired, s::Scope, ui::UserInteraction,
};

use super::Score;
use crate::{Metric, MetricType, v3::Vector};
use core::fmt;

#[cfg(feature = "std")]
use crate::Severity;

/// CVSS v3.x Base Vector. This is actually just an alias for
/// [crate::v3::Vector] for backwards compatibility and should not be used
/// anymore.
pub type Base = Vector;

impl Vector {
    /// Calculate the CVSS (base) score. This is an alias for
    /// [Vector::base_score] for backwards compatibility and should not be used
    /// anymore.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn score(&self) -> Score {
        self.base_score()
    }

    /// Calculate Base CVSS score: overall value for determining the severity
    /// of a vulnerability, generally referred to as the "CVSS score".
    ///
    /// Described in CVSS v3.1 Specification: Section 2:
    /// <https://www.first.org/cvss/specification-document#t6>
    ///
    /// > When the Base metrics are assigned values by an analyst, the Base
    /// > equation computes a score ranging from 0.0 to 10.0.
    /// >
    /// > Specifically, the Base equation is derived from two sub equations:
    /// > the Exploitability sub-score equation, and the Impact sub-score
    /// > equation. The Exploitability sub-score equation is derived from the
    /// > Base Exploitability metrics, while the Impact sub-score equation is
    /// > derived from the Base Impact metrics.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn base_score(&self) -> Score {
        let exploitability = self.exploitability().value();
        let iss = self.impact().value();

        let iss_scoped = if !self.is_scope_changed() {
            6.42 * iss
        } else {
            (7.52 * (iss - 0.029)) - (3.25 * (iss - 0.02).powf(15.0))
        };

        let score = if iss_scoped <= 0.0 {
            0.0
        } else if !self.is_scope_changed() {
            (iss_scoped + exploitability).min(10.0)
        } else {
            (1.08 * (iss_scoped + exploitability)).min(10.0)
        };

        Score::new(score).roundup()
    }

    /// Calculate Base Exploitability score: sub-score for measuring
    /// ease of exploitation.
    ///
    /// Described in CVSS v3.1 Specification: Section 2:
    /// <https://www.first.org/cvss/specification-document#t6>
    ///
    /// > The Exploitability metrics reflect the ease and technical means by which
    /// > the vulnerability can be exploited. That is, they represent characteristics
    /// > of *the thing that is vulnerable*, which we refer to formally as the
    /// > *vulnerable component*.
    pub fn exploitability(&self) -> Score {
        let av_score = self.av.map(|av| av.score()).unwrap_or(0.0);
        let ac_score = self.ac.map(|ac| ac.score()).unwrap_or(0.0);
        let ui_score = self.ui.map(|ui| ui.score()).unwrap_or(0.0);
        let pr_score = self
            .pr
            .map(|pr| pr.scoped_score(self.is_scope_changed()))
            .unwrap_or(0.0);

        (8.22 * av_score * ac_score * pr_score * ui_score).into()
    }

    /// Calculate Base Impact Score (ISS): sub-score for measuring the
    /// consequences of successful exploitation.
    ///
    /// Described in CVSS v3.1 Specification: Section 2:
    /// <https://www.first.org/cvss/specification-document#t6>
    ///
    /// > The Impact metrics reflect the direct consequence
    /// > of a successful exploit, and represent the consequence to the
    /// > *thing that suffers the impact*, which we refer to formally as the
    /// > *impacted component*.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn impact(&self) -> Score {
        let c_score = self.c.map(|c| c.score()).unwrap_or(0.0);
        let i_score = self.i.map(|i| i.score()).unwrap_or(0.0);
        let a_score = self.a.map(|a| a.score()).unwrap_or(0.0);
        (1.0 - ((1.0 - c_score) * (1.0 - i_score) * (1.0 - a_score)).abs()).into()
    }

    /// Iterate over all defined Base metrics
    pub fn metrics(&self) -> impl Iterator<Item = (MetricType, &dyn fmt::Debug)> {
        [
            (
                MetricType::AV,
                self.av.as_ref().map(|m| m as &dyn fmt::Debug),
            ),
            (
                MetricType::AC,
                self.ac.as_ref().map(|m| m as &dyn fmt::Debug),
            ),
            (
                MetricType::PR,
                self.pr.as_ref().map(|m| m as &dyn fmt::Debug),
            ),
            (
                MetricType::UI,
                self.ui.as_ref().map(|m| m as &dyn fmt::Debug),
            ),
            (MetricType::S, self.s.as_ref().map(|m| m as &dyn fmt::Debug)),
            (MetricType::C, self.c.as_ref().map(|m| m as &dyn fmt::Debug)),
            (MetricType::I, self.i.as_ref().map(|m| m as &dyn fmt::Debug)),
            (MetricType::A, self.a.as_ref().map(|m| m as &dyn fmt::Debug)),
        ]
        .into_iter()
        .filter_map(|(name, metric)| metric.as_ref().map(|&m| (name, m)))
    }

    /// Calculate Base CVSS `Severity` according to the
    /// Qualitative Severity Rating Scale (i.e. Low / Medium / High / Critical)
    ///
    /// Described in CVSS v3.1 Specification: Section 5:
    /// <https://www.first.org/cvss/specification-document#t17>
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn severity(&self) -> Severity {
        self.score().severity()
    }

    /// Has the scope changed?
    fn is_scope_changed(&self) -> bool {
        self.s.map(|s| s.is_changed()).unwrap_or(false)
    }
}
