//! CVSS 2.0 Base Metric Group

mod a;
mod ac;
mod au;
mod av;
mod c;
mod i;

pub use self::{
    a::AvailabilityImpact,
    ac::AccessComplexity,
    au::Authentication,
    av::AccessVector,
    c::ConfidentialityImpact,
    i::IntegrityImpact,
};

use super::Score;
use crate::{Error, PREFIX, Result};
use crate::v2::{Metric, MetricType};
use alloc::{borrow::ToOwned, vec::Vec};
use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use {
    alloc::string::{String, ToString},
    serde::{Deserialize, Serialize, de, ser},
};

/// CVSS v2.0 Base Metric Group
///
/// Described in CVSS v2.0 Specification: Section 2.1:
/// <https://www.first.org/cvss/v2/guide#2-1-Base-Metrics>
///
/// > The base metric group captures the characteristics of a vulnerability that
/// > are constant with time and across user environments. The Access Vector,
/// > Access Complexity, and Authentication metrics capture how the
/// > vulnerability is accessed and whether or not extra conditions are required
/// > to exploit it. The three impact metrics measure how a vulnerability, if
/// > exploited, will directly affect an IT asset, where the impacts are
/// > independently defined as the degree of loss of confidentiality, integrity,
/// > and availability. For example, a vulnerability could cause a partial loss
/// > of integrity and availability, but no loss of confidentiality.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Base {
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
}

impl Base {
    /// Calculate Base CVSS score: overall value for determining the severity
    /// of a vulnerability, generally referred to as the "CVSS score".
    ///
    /// Described in CVSS v2.0 Specification: Section 3.2.1:
    /// <https://www.first.org/cvss/v2/guide#3-2-1-Base-Equation>
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn score(&self) -> Score {
        let exploitability = self.exploitability().value();
        let impact = self.impact().value();

        let f_impact = if impact == 0.0 {
            0.0
        } else {
            1.176
        };

        let score = ((0.6*impact)+(0.4*exploitability)-1.5)*f_impact;

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
        (10.41 * (1.0 - ((1.0 - c_score) * (1.0 - i_score) * (1.0 - a_score)).abs())).into()
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

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_metrics!(
            f, self.av, self.ac, self.au, self.c, self.i, self.a
        );
        Ok(())
    }
}

impl FromStr for Base {
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
            ..Default::default()
        };

        for &component in components {
            let id = component.0.to_ascii_uppercase();
            let value = component.1.to_ascii_uppercase();

            match id.parse::<MetricType>()? {
                MetricType::AV => metrics.av = Some(value.parse()?),
                MetricType::AC => metrics.ac = Some(value.parse()?),
                MetricType::Au => metrics.au = Some(value.parse()?),
                MetricType::C => metrics.c = Some(value.parse()?),
                MetricType::I => metrics.i = Some(value.parse()?),
                MetricType::A => metrics.a = Some(value.parse()?),
            }
        }

        Ok(metrics)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Base {
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
impl Serialize for Base {
    fn serialize<S: ser::Serializer>(
        &self,
        serializer: S,
    ) -> core::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
