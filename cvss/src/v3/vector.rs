use crate::v3::base::{
    AttackComplexity, AttackVector, Availability, Confidentiality, Integrity, PrivilegesRequired,
    Scope, UserInteraction,
};
use crate::v3::environmental::{
    AvailabilityRequirement, ConfidentialityRequirement, IntegrityRequirement,
    ModifiedAttackComplexity, ModifiedAttackVector, ModifiedAvailability, ModifiedConfidentiality,
    ModifiedIntegrity, ModifiedPrivilegesRequired, ModifiedScope, ModifiedUserInteraction,
};
use crate::v3::temporal::{ExploitCodeMaturity, RemediationLevel, ReportConfidence};
use crate::{Error, MetricType, PREFIX, Result};
use alloc::{borrow::ToOwned, vec::Vec};
use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use {
    alloc::string::{String, ToString},
    serde::{Deserialize, Serialize, de, ser},
};

/// A CVSS 3.x vector, including Base, Temporal, and Environmental metrics.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Vector {
    /// Minor component of the version
    pub minor_version: usize,

    /// Attack Vector (AV)
    pub av: Option<AttackVector>,

    /// Attack Complexity (AC)
    pub ac: Option<AttackComplexity>,

    /// Privileges Required (PR)
    pub pr: Option<PrivilegesRequired>,

    /// User Interaction (UI)
    pub ui: Option<UserInteraction>,

    /// Scope (S)
    pub s: Option<Scope>,

    /// Confidentiality Impact (C)
    pub c: Option<Confidentiality>,

    /// Integrity Impact (I)
    pub i: Option<Integrity>,

    /// Availability Impact (A)
    pub a: Option<Availability>,

    /// Exploit Code Maturity (E)
    pub e: Option<ExploitCodeMaturity>,

    /// Remediation Level (RL)
    pub rl: Option<RemediationLevel>,

    /// Report Confidence (RC)
    pub rc: Option<ReportConfidence>,

    /// Modified Attack Vector (MAV)
    pub mav: Option<ModifiedAttackVector>,

    /// Confidentiality Requirements (CR)
    pub cr: Option<ConfidentialityRequirement>,

    /// Integrity Requirements (IR)
    pub ir: Option<IntegrityRequirement>,

    /// Availability Requirements (AR)
    pub ar: Option<AvailabilityRequirement>,

    /// Modified Attack Complexity (MAC)
    pub mac: Option<ModifiedAttackComplexity>,

    /// Modified Privileges Required (MPR)
    pub mpr: Option<ModifiedPrivilegesRequired>,

    /// Modified User Interaction (MUI)
    pub mui: Option<ModifiedUserInteraction>,

    /// Modified Scope (MS)
    pub ms: Option<ModifiedScope>,

    /// Modified Confidentiality (MC)
    pub mc: Option<ModifiedConfidentiality>,

    /// Modified Integrity (MI)
    pub mi: Option<ModifiedIntegrity>,

    /// Modified Availability (MA)
    pub ma: Option<ModifiedAvailability>,
}

// Helper macro to build the array of (MetricType, Option<&dyn fmt::Debug>)
macro_rules! metrics_array {
    ($s:expr, $( ($metric_ty:expr, $field:ident) ),+ $(,)?) => {
        [
            $(
                ($metric_ty, $s.$field.as_ref().map(|m| m as &dyn fmt::Debug)),
            )+
        ]
    };
}

impl Vector {
    /// Iterate over all defined metrics in this vector
    pub fn metrics(&self) -> impl Iterator<Item = (MetricType, &dyn fmt::Debug)> {
        metrics_array!(
            self,
            (MetricType::AV, av),
            (MetricType::AC, ac),
            (MetricType::PR, pr),
            (MetricType::UI, ui),
            (MetricType::S, s),
            (MetricType::C, c),
            (MetricType::I, i),
            (MetricType::A, a),
            // Temporal metrics
            (MetricType::E, e),
            (MetricType::RL, rl),
            (MetricType::RC, rc),
            // Environmental metrics
            (MetricType::MAV, mav),
            (MetricType::MAC, mac),
            (MetricType::MPR, mpr),
            (MetricType::MUI, mui),
            (MetricType::MS, ms),
            (MetricType::MC, mc),
            (MetricType::MI, mi),
            (MetricType::CR, cr),
            (MetricType::IR, ir),
            (MetricType::AR, ar),
            (MetricType::MA, ma),
        )
        .into_iter()
        .filter_map(|(name, metric)| metric.as_ref().map(|&m| (name, m)))
    }
}

macro_rules! write_metrics {
    ($f:expr, $($metric:expr),+) => {
        $(
            if let Some(metric) = $metric {
                write!($f, "/{}", metric)?;
            }
        )+
    };
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:3.{}", PREFIX, self.minor_version)?;
        write_metrics!(
            f, self.av, self.ac, self.pr, self.ui, self.s, self.c, self.i, self.a,
            // Temporal
            self.e, self.rl, self.rc, // Requirements (standard order)
            self.cr, self.ir, self.ar, // Modified base metrics
            self.mav, self.mac, self.mpr, self.mui, self.ms, self.mc, self.mi, self.ma
        );
        Ok(())
    }
}

impl FromStr for Vector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let component_vec = s
            .split('/')
            .map(|component| {
                let mut parts = component.split(':');

                let id = parts.next().ok_or_else(|| Error::InvalidComponent {
                    component: component.to_owned(),
                })?;

                let value = parts.next().ok_or_else(|| Error::InvalidComponent {
                    component: component.to_owned(),
                })?;

                if parts.next().is_some() {
                    return Err(Error::InvalidComponent {
                        component: component.to_owned(),
                    });
                }

                Ok((id, value))
            })
            .collect::<Result<Vec<_>>>()?;

        let mut components = component_vec.iter();
        let &(id, version_string) = components.next().ok_or(Error::InvalidPrefix {
            prefix: s.to_owned(),
        })?;

        if id != PREFIX {
            return Err(Error::InvalidPrefix {
                prefix: id.to_owned(),
            });
        }

        let mut metrics = Self {
            minor_version: match version_string {
                "3.0" => 0,
                "3.1" => 1,
                _ => {
                    return Err(Error::UnsupportedVersion {
                        version: version_string.to_owned(),
                    });
                }
            },
            ..Default::default()
        };

        for &component in components {
            let id = component.0.to_ascii_uppercase();
            let value = component.1.to_ascii_uppercase();

            match id.parse::<MetricType>()? {
                // Base metrics
                MetricType::AV => metrics.av = Some(value.parse()?),
                MetricType::AC => metrics.ac = Some(value.parse()?),
                MetricType::PR => metrics.pr = Some(value.parse()?),
                MetricType::UI => metrics.ui = Some(value.parse()?),
                MetricType::S => metrics.s = Some(value.parse()?),
                MetricType::C => metrics.c = Some(value.parse()?),
                MetricType::I => metrics.i = Some(value.parse()?),
                MetricType::A => metrics.a = Some(value.parse()?),

                // Temporal metrics
                MetricType::E => metrics.e = Some(value.parse()?),
                MetricType::RL => metrics.rl = Some(value.parse()?),
                MetricType::RC => metrics.rc = Some(value.parse()?),

                // Environmental metrics (use constructors that accept the base metric)
                MetricType::MAV => metrics.mav = Some(value.parse()?),
                MetricType::MAC => metrics.mac = Some(value.parse()?),
                MetricType::MPR => metrics.mpr = Some(value.parse()?),
                MetricType::MUI => metrics.mui = Some(value.parse()?),
                MetricType::MS => metrics.ms = Some(value.parse()?),
                MetricType::MC => metrics.mc = Some(value.parse()?),
                MetricType::MI => metrics.mi = Some(value.parse()?),
                MetricType::MA => metrics.ma = Some(value.parse()?),
                MetricType::CR => metrics.cr = Some(value.parse()?),
                MetricType::IR => metrics.ir = Some(value.parse()?),
                MetricType::AR => metrics.ar = Some(value.parse()?),
            }
        }

        Ok(metrics)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Vector {
    fn deserialize<D: de::Deserializer<'de>>(
        deserializer: D,
    ) -> core::result::Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Vector {
    fn serialize<S: ser::Serializer>(
        &self,
        serializer: S,
    ) -> core::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::v3::{Vector, environmental};
    use core::str::FromStr;
    use std::string::ToString;

    #[test]
    fn parse_full_cvss3() {
        // See https://www.first.org/cvss/calculator/3-1#CVSS:3.1/AV:A/AC:H/PR:L/UI:N/S:U/C:L/I:L/A:N/E:P/RL:T/RC:R/AR:H/MAC:H/MUI:R/MS:C/MC:L/MA:N
        let vector_s = "CVSS:3.1/AV:A/AC:H/PR:L/UI:N/S:U/C:L/I:L/A:N/E:P/RL:T/RC:R/CR:X/IR:X/AR:H/MAV:X/MAC:H/MPR:X/MUI:R/MS:C/MC:L/MI:X/MA:N";
        let v: Vector = Vector::from_str(vector_s).unwrap();
        assert_eq!(vector_s, v.to_string());

        let base_score = v.base_score().value();
        assert_eq!(base_score, 3.7);

        let temporal_score = v.temporal_score().value();
        assert_eq!(temporal_score, 3.3);

        let environmental_score = v.environmental_score().value();
        assert_eq!(environmental_score, 3.5);
    }

    #[test]
    fn cvss30_vs_cvss31() {
        // See https://www.first.org/cvss/calculator/3-0#CVSS:3.0/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:H
        let v30 = "CVSS:3.0/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:H";
        // See https://www.first.org/cvss/calculator/3-1#CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:H
        let v31 = "CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:H/I:H/A:H";

        let vec30: Vector = Vector::from_str(v30).unwrap();
        let vec31: Vector = Vector::from_str(v31).unwrap();

        let base_score_30 = vec30.base_score().value();
        let base_score_31 = vec31.base_score().value();
        assert_eq!(base_score_30, base_score_31);

        let temporal_score_30 = vec30.temporal_score().value();
        let temporal_score_31 = vec31.temporal_score().value();
        assert_eq!(temporal_score_30, temporal_score_31);

        // Environmental scores are different between CVSS v3.0 and v3.1 for
        // this vector because of the different exponent used in the calculation
        // of the Modified Impact sub-score when Scope is changed.
        let environmental_score_30 = vec30.environmental_score().value();
        let environmental_score_31 = vec31.environmental_score().value();

        assert_ne!(environmental_score_30, environmental_score_31);
        assert_eq!(environmental_score_30, 9.6);
        assert_eq!(environmental_score_31, 9.7);
    }
}
