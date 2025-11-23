use crate::v2::MetricType;
use crate::v2::base::{
    AccessComplexity, AccessVector, Authentication, AvailabilityImpact, ConfidentialityImpact,
    IntegrityImpact,
};
use crate::v2::temporal::{Exploitability, RemediationLevel, ReportConfidence};
use crate::{Error, Result};
use alloc::{borrow::ToOwned, vec::Vec};
use core::fmt;
use core::str::FromStr;
#[cfg(feature = "serde")]
use {
    alloc::string::{String, ToString},
    serde::{Deserialize, Serialize, de, ser},
};

/// CVSS v2.0 Vector
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Vector {
    /// Access Vector (AV)
    pub av: Option<AccessVector>,

    /// Access Complexity (AC)
    pub ac: Option<AccessComplexity>,

    /// Authentication (Au)
    pub au: Option<Authentication>,

    /// Confidentiality Impact (C)
    pub c: Option<ConfidentialityImpact>,

    /// Integrity Impact (I)
    pub i: Option<IntegrityImpact>,

    /// Availability Impact (A)
    pub a: Option<AvailabilityImpact>,

    /// Exploitability (E)
    pub e: Option<Exploitability>,

    /// Remediation Level (RL)
    pub rl: Option<RemediationLevel>,

    /// Report Confidence (RC)
    pub rc: Option<ReportConfidence>,
}

macro_rules! write_metrics {
    ($f:expr, $($metric:expr),+) => {
        let mut __first = true;
        $(
            if let Some(metric) = $metric {
                if __first {
                    write!($f, "{}", metric)?;
                    __first = false;
                } else {
                    write!($f, "/{}", metric)?;
                }
            }
        )+
    };
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_metrics!(f, self.av, self.ac, self.au, self.c, self.i, self.a);
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

        let components = component_vec.iter();
        let mut metrics = Self {
            ..Default::default()
        };

        for &component in components {
            let key = component.0;
            let value = component.1;

            match key.parse::<MetricType>()? {
                // Base metrics
                MetricType::AV => metrics.av = Some(value.parse()?),
                MetricType::AC => metrics.ac = Some(value.parse()?),
                MetricType::Au => metrics.au = Some(value.parse()?),
                MetricType::C => metrics.c = Some(value.parse()?),
                MetricType::I => metrics.i = Some(value.parse()?),
                MetricType::A => metrics.a = Some(value.parse()?),

                // Temporal metrics
                MetricType::E => metrics.e = Some(value.parse()?),
                MetricType::RL => metrics.rl = Some(value.parse()?),
                MetricType::RC => metrics.rc = Some(value.parse()?),

                _ => return Err(Error::InvalidComponent {
                    component: key.to_owned(),
                }),                
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
