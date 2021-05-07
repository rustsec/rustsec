//! CVSS v3.1 Base Metric Group

pub mod a;
pub mod ac;
pub mod av;
pub mod c;
pub mod i;
pub mod pr;
pub mod s;
pub mod ui;

use super::{metric::Metric, Score};
use crate::{
    error::{Error, ErrorKind},
    Severity, PREFIX,
};
#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub use self::{
    a::Availability, ac::AttackComplexity, av::AttackVector, c::Confidentiality, i::Integrity,
    pr::PrivilegesRequired, s::Scope, ui::UserInteraction,
};

/// CVSS v3.1 Base Metric Group
///
/// Described in CVSS v3.1 Specification: Section 2:
/// <https://www.first.org/cvss/specification-document#t6>
///
/// > The Base metric group represents the intrinsic characteristics of a
/// > vulnerability that are constant over time and across user environments.
/// > It is composed of two sets of metrics: the Exploitability metrics and
/// > the Impact metrics.
/// >
/// > The Exploitability metrics reflect the ease and technical means by which
/// > the vulnerability can be exploited. That is, they represent characteristics
/// > of *the thing that is vulnerable*, which we refer to formally as the
/// > *vulnerable component*. The Impact metrics reflect the direct consequence
/// > of a successful exploit, and represent the consequence to the
/// > *thing that suffers the impact*, which we refer to formally as the
/// > *impacted component*.
/// >
/// > While the vulnerable component is typically a software application,
/// > module, driver, etc. (or possibly a hardware device), the impacted
/// > component could be a software application, a hardware device or a network
/// > resource. This potential for measuring the impact of a vulnerability other
/// > than the vulnerable component, was a key feature introduced with
/// > CVSS v3.0. This property is captured by the Scope metric.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Base {
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
}

impl Base {
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
    pub fn score(&self) -> Score {
        let exploitability = self.exploitability().value();
        let iss = self.impact().value();

        let iss_scoped = if !self.is_scope_changed() {
            6.42 * iss
        } else {
            (7.52 * (iss - 0.029).abs()) - (3.25 * (iss - 0.02).abs().powf(15.0))
        };

        let score = if iss_scoped < 0.0 {
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
    pub fn impact(&self) -> Score {
        let c_score = self.c.map(|c| c.score()).unwrap_or(0.0);
        let i_score = self.i.map(|i| i.score()).unwrap_or(0.0);
        let a_score = self.a.map(|a| a.score()).unwrap_or(0.0);
        (1.0 - ((1.0 - c_score) * (1.0 - i_score) * (1.0 - a_score)).abs()).into()
    }

    /// Calculate Base CVSS `Severity` according to the
    /// Qualitative Severity Rating Scale (i.e. Low / Medium / High / Critical)
    ///
    /// Described in CVSS v3.1 Specification: Section 5:
    /// <https://www.first.org/cvss/specification-document#t17>
    pub fn severity(&self) -> Severity {
        self.score().severity()
    }

    /// Has the scope changed?
    fn is_scope_changed(&self) -> bool {
        self.s.map(|s| s.is_changed()).unwrap_or(false)
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
        write!(f, "{}:3.{}", PREFIX, self.minor_version)?;
        write_metrics!(f, self.av, self.ac, self.pr, self.ui, self.s, self.c, self.i, self.a);
        Ok(())
    }
}

impl FromStr for Base {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut components = s.split('/').map(|component| {
            let mut parts = component.split(':');

            let id = parts.next().ok_or_else(|| {
                format_err!(ErrorKind::Parse, "empty component in CVSS vector: {}", s)
            })?;

            let value = parts.next().ok_or_else(|| {
                format_err!(
                    ErrorKind::Parse,
                    "empty value for CVSS vector component: {}",
                    id
                )
            })?;

            if parts.next().is_some() {
                fail!(
                    ErrorKind::Parse,
                    "malformed CVSS vector component: {}",
                    component
                );
            }

            Ok((id, value))
        });

        let prefix = components
            .next()
            .ok_or_else(|| format_err!(ErrorKind::Parse, "empty CVSS string"))?;

        let (id, version_string) = prefix?;

        if id != PREFIX {
            fail!(ErrorKind::Parse, "invalid CVSS prefix: {}", id);
        }

        let mut metrics = Self {
            minor_version: match version_string {
                "3.0" => 0,
                "3.1" => 1,
                _ => fail!(
                    ErrorKind::Version,
                    "wrong CVSS version (expected one of '3.0' or '3.1'): '{}'",
                    version_string
                ),
            },
            ..Default::default()
        };

        for component in components {
            let component = component?;
            let id = component.0.to_ascii_uppercase();
            let value = component.1.to_ascii_uppercase();

            match id.as_str() {
                "AV" => metrics.av = Some(value.parse()?),
                "AC" => metrics.ac = Some(value.parse()?),
                "PR" => metrics.pr = Some(value.parse()?),
                "UI" => metrics.ui = Some(value.parse()?),
                "S" => metrics.s = Some(value.parse()?),
                "C" => metrics.c = Some(value.parse()?),
                "I" => metrics.i = Some(value.parse()?),
                "A" => metrics.a = Some(value.parse()?),
                other => fail!(ErrorKind::Parse, "unknown metric type: '{}'", other),
            }
        }

        Ok(metrics)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Base {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Base {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
