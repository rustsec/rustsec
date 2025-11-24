use crate::v3::base::{
    AttackComplexity, AttackVector, Availability, Confidentiality, Integrity, PrivilegesRequired,
    Scope, UserInteraction,
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
    e: Option<ExploitCodeMaturity>,

    /// Remediation Level (RL)
    rl: Option<RemediationLevel>,

    /// Report Confidence (RC)
    rc: Option<ReportConfidence>,
}

impl Vector {
    /// Iterate over all defined metrics in this vector
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
            (MetricType::E, self.e.as_ref().map(|m| m as &dyn fmt::Debug)),
            (
                MetricType::RL,
                self.rl.as_ref().map(|m| m as &dyn fmt::Debug),
            ),
            (
                MetricType::RC,
                self.rc.as_ref().map(|m| m as &dyn fmt::Debug),
            ),
        ]
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
            f, self.av, self.ac, self.pr, self.ui, self.s, self.c, self.i, self.a
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
