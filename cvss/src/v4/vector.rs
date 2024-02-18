//! Score computing for CVSSv4

use crate::{
    v4::{
        metric::{
            base::{
                AttackComplexity, AttackRequirements, AttackVector,
                AvailabilityImpactToTheSubsequentSystem, AvailabilityImpactToTheVulnerableSystem,
                ConfidentialityImpactToTheSubsequentSystem,
                ConfidentialityImpactToTheVulnerableSystem, IntegrityImpactToTheSubsequentSystem,
                IntegrityImpactToTheVulnerableSystem, PrivilegesRequired, UserInteraction,
            },
            environmental::{
                AvailabilityRequirements, ConfidentialityRequirements, IntegrityRequirements,
                ModifiedAttackComplexity, ModifiedAttackRequirements, ModifiedAttackVector,
                ModifiedAvailabilityImpactToTheSubsequentSystem,
                ModifiedAvailabilityImpactToTheVulnerableSystem,
                ModifiedConfidentialityImpactToTheSubsequentSystem,
                ModifiedConfidentialityImpactToTheVulnerableSystem,
                ModifiedIntegrityImpactToTheSubsequentSystem,
                ModifiedIntegrityImpactToTheVulnerableSystem, ModifiedPrivilegesRequired,
                ModifiedUserInteraction,
            },
            supplemental::{
                Automatable, ProviderUrgency, Recovery, Safety, ValueDensity,
                VulnerabilityResponseEffort,
            },
            threat::ExploitMaturity,
        },
        MetricTypeV4,
    },
    Error, PREFIX,
};
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::{fmt, str::FromStr};
#[cfg(feature = "serde")]
use {
    alloc::string::ToString,
    serde::{de, ser, Deserialize, Serialize},
};

use crate::v4::score::Nomenclature;
#[cfg(feature = "std")]
use crate::v4::ScoreV4;

/// A CVSS 4.0 vector
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Vector {
    /// Minor component of the version
    pub minor_version: usize,

    /// Attack Complexity (AC)
    pub(crate) ac: Option<AttackComplexity>,
    /// Attack Requirements (AT)
    pub(crate) at: Option<AttackRequirements>,
    /// Attack Vector (AV)
    pub(crate) av: Option<AttackVector>,
    /// Privileges Required (PR)
    pub(crate) pr: Option<PrivilegesRequired>,
    /// Availability Impact to the Subsequent System (SA)
    pub(crate) sa: Option<AvailabilityImpactToTheSubsequentSystem>,
    /// Confidentiality Impact to the Subsequent System (SC)
    pub(crate) sc: Option<ConfidentialityImpactToTheSubsequentSystem>,
    /// Integrity Impact to the Subsequent System (SI)
    pub(crate) si: Option<IntegrityImpactToTheSubsequentSystem>,
    /// User Interaction (UI)
    pub(crate) ui: Option<UserInteraction>,
    /// Availability Impact to the Vulnerable System (VA)
    pub(crate) va: Option<AvailabilityImpactToTheVulnerableSystem>,
    /// Confidentiality Impact to the Vulnerable System (VC)
    pub(crate) vc: Option<ConfidentialityImpactToTheVulnerableSystem>,
    /// Integrity Impact to the Vulnerable System (VI)
    pub(crate) vi: Option<IntegrityImpactToTheVulnerableSystem>,
    /// Exploit Maturity (E)
    pub(crate) e: Option<ExploitMaturity>,
    /// Availability Requirements (AR)
    pub(crate) ar: Option<AvailabilityRequirements>,
    /// Confidentiality Requirements (CR)
    pub(crate) cr: Option<ConfidentialityRequirements>,
    /// Integrity Requirements (IR)
    pub(crate) ir: Option<IntegrityRequirements>,
    /// Modified Attack Complexity (AC)
    pub(crate) mac: Option<ModifiedAttackComplexity>,
    /// Modified Attack Requirements (MAT)
    pub(crate) mat: Option<ModifiedAttackRequirements>,
    /// Modified Attack Vector (MAV)
    pub(crate) mav: Option<ModifiedAttackVector>,
    /// Modified Privileges Required (MPR)
    pub(crate) mpr: Option<ModifiedPrivilegesRequired>,
    /// Modified Availability Impact to the Subsequent System (MSA)
    pub(crate) msa: Option<ModifiedAvailabilityImpactToTheSubsequentSystem>,
    /// Modified Confidentiality Impact to the Subsequent System (MSC)
    pub(crate) msc: Option<ModifiedConfidentialityImpactToTheSubsequentSystem>,
    /// Modified Integrity Impact to the Subsequent System (MSI)
    pub(crate) msi: Option<ModifiedIntegrityImpactToTheSubsequentSystem>,
    /// Modified User Interaction (MUI)
    pub(crate) mui: Option<ModifiedUserInteraction>,
    /// Modified Availability Impact to the Vulnerable System (MVA)
    pub(crate) mva: Option<ModifiedAvailabilityImpactToTheVulnerableSystem>,
    /// Modified Confidentiality Impact to the Vulnerable System (MVC)
    pub(crate) mvc: Option<ModifiedConfidentialityImpactToTheVulnerableSystem>,
    /// Modified Integrity Impact to the Vulnerable System (MVI)
    pub(crate) mvi: Option<ModifiedIntegrityImpactToTheVulnerableSystem>,
    /// Automatable (AU)
    pub(crate) au: Option<Automatable>,
    /// Recovery (R)
    pub(crate) r: Option<Recovery>,
    /// Vulnerability Response Effort (RE)
    pub(crate) re: Option<VulnerabilityResponseEffort>,
    /// Safety (S)
    pub(crate) s: Option<Safety>,
    /// Provider Urgency (U)
    pub(crate) u: Option<ProviderUrgency>,
    /// Value Density (V)
    pub(crate) v: Option<ValueDensity>,
}

impl Vector {
    /// Get the numerical score of the vector
    #[cfg(feature = "std")]
    pub fn score(&self) -> ScoreV4 {
        self.into()
    }

    /// Get the nomenclature of the vector
    ///
    /// This nomenclature should be used wherever a numerical CVSS value is displayed or communicated.
    pub fn nomenclature(&self) -> Nomenclature {
        Nomenclature::from(self)
    }

    /// Check for required base metrics presence
    ///
    /// Defined in <https://www.first.org/cvss/v4.0/specification-document#Vector-String>
    fn check_mandatory_metrics(&self) -> Result<(), Error> {
        fn ensure_present<T>(metric: Option<T>, metric_type: MetricTypeV4) -> Result<(), Error> {
            if metric.is_none() {
                return Err(Error::MissingMandatoryMetricV4 { metric_type });
            }
            Ok(())
        }

        ensure_present(self.ac.as_ref(), MetricTypeV4::AC)?;
        ensure_present(self.at.as_ref(), MetricTypeV4::AT)?;
        ensure_present(self.av.as_ref(), MetricTypeV4::AV)?;
        ensure_present(self.pr.as_ref(), MetricTypeV4::PR)?;
        ensure_present(self.sa.as_ref(), MetricTypeV4::SA)?;
        ensure_present(self.sc.as_ref(), MetricTypeV4::SC)?;
        ensure_present(self.si.as_ref(), MetricTypeV4::SI)?;
        ensure_present(self.ui.as_ref(), MetricTypeV4::UI)?;
        ensure_present(self.va.as_ref(), MetricTypeV4::VA)?;
        ensure_present(self.vc.as_ref(), MetricTypeV4::VC)?;
        ensure_present(self.vi.as_ref(), MetricTypeV4::VI)?;
        Ok(())
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
        write!(f, "{}:4.{}", PREFIX, self.minor_version)?;
        write_metrics!(
            f, self.av, self.ac, self.at, self.pr, self.ui, self.vc, self.vi, self.va, self.sc,
            self.si, self.sa, self.e, self.cr, self.ir, self.ar, self.mav, self.mac, self.mat,
            self.mpr, self.mui, self.mvc, self.mvi, self.mva, self.msc, self.msi, self.msa, self.s,
            self.au, self.r, self.v, self.re, self.u
        );
        Ok(())
    }
}

impl FromStr for Vector {
    type Err = Error;

    fn from_str(s: &str) -> crate::Result<Self> {
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
            .collect::<crate::Result<Vec<_>>>()?;

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
                "4.0" => 0,
                _ => {
                    return Err(Error::UnsupportedVersion {
                        version: version_string.to_owned(),
                    })
                }
            },
            ..Default::default()
        };

        for &component in components {
            let id = component.0.to_ascii_uppercase();
            let value = component.1.to_ascii_uppercase();

            fn get_value<T: FromStr<Err = Error>>(
                metric_type: MetricTypeV4,
                current_val: Option<T>,
                new_val: String,
            ) -> Result<Option<T>, Error> {
                let parsed: T = new_val.parse()?;
                if current_val.is_some() {
                    return Err(Error::DuplicateMetricV4 { metric_type });
                }
                Ok(Some(parsed))
            }

            match id.parse::<MetricTypeV4>()? {
                MetricTypeV4::AV => metrics.av = get_value(MetricTypeV4::AV, metrics.av, value)?,
                MetricTypeV4::AC => metrics.ac = get_value(MetricTypeV4::AC, metrics.ac, value)?,
                MetricTypeV4::PR => metrics.pr = get_value(MetricTypeV4::PR, metrics.pr, value)?,
                MetricTypeV4::UI => metrics.ui = get_value(MetricTypeV4::UI, metrics.ui, value)?,
                MetricTypeV4::S => metrics.s = get_value(MetricTypeV4::S, metrics.s, value)?,
                MetricTypeV4::AT => metrics.at = get_value(MetricTypeV4::AT, metrics.at, value)?,
                MetricTypeV4::SA => metrics.sa = get_value(MetricTypeV4::SA, metrics.sa, value)?,
                MetricTypeV4::SC => metrics.sc = get_value(MetricTypeV4::SC, metrics.sc, value)?,
                MetricTypeV4::SI => metrics.si = get_value(MetricTypeV4::SI, metrics.si, value)?,
                MetricTypeV4::VA => metrics.va = get_value(MetricTypeV4::VA, metrics.va, value)?,
                MetricTypeV4::VC => metrics.vc = get_value(MetricTypeV4::VC, metrics.vc, value)?,
                MetricTypeV4::VI => metrics.vi = get_value(MetricTypeV4::VI, metrics.vi, value)?,
                MetricTypeV4::E => metrics.e = get_value(MetricTypeV4::E, metrics.e, value)?,
                MetricTypeV4::AR => metrics.ar = get_value(MetricTypeV4::AR, metrics.ar, value)?,
                MetricTypeV4::CR => metrics.cr = get_value(MetricTypeV4::CR, metrics.cr, value)?,
                MetricTypeV4::IR => metrics.ir = get_value(MetricTypeV4::IR, metrics.ir, value)?,
                MetricTypeV4::MAC => {
                    metrics.mac = get_value(MetricTypeV4::MAC, metrics.mac, value)?
                }
                MetricTypeV4::MAT => {
                    metrics.mat = get_value(MetricTypeV4::MAT, metrics.mat, value)?
                }
                MetricTypeV4::MAV => {
                    metrics.mav = get_value(MetricTypeV4::MAV, metrics.mav, value)?
                }
                MetricTypeV4::MPR => {
                    metrics.mpr = get_value(MetricTypeV4::MPR, metrics.mpr, value)?
                }
                MetricTypeV4::MSA => {
                    metrics.msa = get_value(MetricTypeV4::MSA, metrics.msa, value)?
                }
                MetricTypeV4::MSC => {
                    metrics.msc = get_value(MetricTypeV4::MSC, metrics.msc, value)?
                }
                MetricTypeV4::MSI => {
                    metrics.msi = get_value(MetricTypeV4::MSI, metrics.msi, value)?
                }
                MetricTypeV4::MUI => {
                    metrics.mui = get_value(MetricTypeV4::MUI, metrics.mui, value)?
                }
                MetricTypeV4::MVA => {
                    metrics.mva = get_value(MetricTypeV4::MVA, metrics.mva, value)?
                }
                MetricTypeV4::MVC => {
                    metrics.mvc = get_value(MetricTypeV4::MVC, metrics.mvc, value)?
                }
                MetricTypeV4::MVI => {
                    metrics.mvi = get_value(MetricTypeV4::MVI, metrics.mvi, value)?
                }
                MetricTypeV4::AU => metrics.au = get_value(MetricTypeV4::AU, metrics.au, value)?,
                MetricTypeV4::R => metrics.r = get_value(MetricTypeV4::R, metrics.r, value)?,
                MetricTypeV4::RE => metrics.re = get_value(MetricTypeV4::RE, metrics.re, value)?,
                MetricTypeV4::U => metrics.u = get_value(MetricTypeV4::U, metrics.u, value)?,
                MetricTypeV4::V => metrics.v = get_value(MetricTypeV4::V, metrics.v, value)?,
            }
        }

        metrics.check_mandatory_metrics()?;

        Ok(metrics)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Vector {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Vector {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use alloc::{borrow::ToOwned, string::ToString};

    #[test]
    fn fails_to_parse_invalid_cvss4() {
        // Version 5.0 is not supported
        assert_eq!(
            Vector::from_str("CVSS:5.0/AV:N/AC:L/AT:N/PR:H/UI:N/VC:L/VI:L/VA:N/SC:N/SI:N/SA:N"),
            Err(Error::UnsupportedVersion {
                version: "5.0".to_string(),
            })
        );
        // Invalid prefix CSS
        assert_eq!(
            Vector::from_str("CSS:4.0/AV:N/AC:L/AT:N/PR:H/UI:N/VC:L/VI:L/VA:N/SC:N/SI:N/SA:N"),
            Err(Error::InvalidPrefix {
                prefix: "CSS".to_owned(),
            })
        );
        // “F” is not a valid value for “AV”
        assert_eq!(
            Vector::from_str("CVSS:4.0/AV:F/AC:L/AT:N/PR:N/UI:N/VC:N/VI:L/VA:N/SC:N/SI:N/SA:N"),
            Err(Error::InvalidMetricV4 {
                metric_type: MetricTypeV4::AV,
                value: "F".to_owned()
            })
        );
        // Missing mandatory metric “AC”
        assert_eq!(
            Vector::from_str("CVSS:4.0/AV:N/AT:N/PR:H/UI:N/VC:L/VI:L/VA:N/SC:N/SI:N/SA:N"),
            Err(Error::MissingMandatoryMetricV4 {
                metric_type: MetricTypeV4::AC
            })
        );
    }

    #[test]
    fn parse_base_cvss4() {
        assert!(Vector::from_str(
            "CVSS:4.0/AV:N/AC:L/AT:N/PR:H/UI:N/VC:L/VI:L/VA:N/SC:N/SI:N/SA:N"
        )
        .is_ok());
    }

    #[test]
    fn parse_full_cvss4() {
        let vector_s = "CVSS:4.0/AV:N/AC:L/AT:N/PR:H/UI:N/VC:L/VI:L/VA:N/SC:N/SI:N/SA:N/E:U/CR:L/IR:X/AR:L/MAV:A/MAC:H/MAT:N/MPR:N/MUI:P/MVC:X/MVI:N/MVA:H/MSC:N/MSI:L/MSA:S/S:N/AU:N/R:I/V:C/RE:H/U:Green";
        let v = Vector::from_str(vector_s).unwrap();
        assert_eq!(vector_s, v.to_string());
    }
}
