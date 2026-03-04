use crate::v2::MetricType;
use crate::v2::base::{
    AccessComplexity, AccessVector, Authentication, AvailabilityImpact, ConfidentialityImpact,
    IntegrityImpact,
};
use crate::v2::environmental::{
    AvailabilityRequirement, CollateralDamagePotential, ConfidentialityRequirement,
    IntegrityRequirement, TargetDistribution,
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

    /// Collateral Damage Potential (CDP)
    pub cdp: Option<CollateralDamagePotential>,

    /// Target Distribution (TD)
    pub td: Option<TargetDistribution>,

    /// Confidentiality Requirement (CR)
    pub cr: Option<ConfidentialityRequirement>,

    /// Integrity Requirement (IR)
    pub ir: Option<IntegrityRequirement>,

    /// Availability Requirement (AR)
    pub ar: Option<AvailabilityRequirement>,
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
        write_metrics!(
            f, self.av, self.ac, self.au, self.c, self.i, self.a, self.e, self.rl, self.rc,
            self.cdp, self.td, self.cr, self.ir, self.ar
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

                // Environmental metrics
                MetricType::CDP => metrics.cdp = Some(value.parse()?),
                MetricType::TD => metrics.td = Some(value.parse()?),
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
    use super::Vector;
    use core::str::FromStr;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.0001
    }

    #[test]
    fn spec_cve_2002_0392() {
        // See https://nvd.nist.gov/vuln-metrics/cvss/v2-calculator?vector=(AV:N/AC:L/Au:N/C:N/I:N/A:C/E:F/RL:OF/RC:C/CDP:H/TD:H/CR:M/IR:M/AR:H)
        let v = "AV:N/AC:L/Au:N/C:N/I:N/A:C/E:F/RL:OF/RC:C/CDP:H/TD:H/CR:M/IR:M/AR:H";
        let cvss = Vector::from_str(v).expect("parse vector");

        let base = cvss.base_score().value();
        let impact = cvss.impact().roundup().value();
        let exploitability = cvss.exploitability().roundup().value();
        let temporal = cvss.temporal_score().value();
        let environmental = cvss.environmental_score().value();
        let adjusted_impact = cvss.adjusted_impact().roundup().value();
        assert!(
            approx_eq(base, 7.8),
            "base score expected 7.8, got {}",
            base
        );
        assert!(
            approx_eq(impact, 6.9),
            "impact expected 6.9, got {}",
            impact
        );
        assert!(
            approx_eq(exploitability, 10.0),
            "exploitability expected 10.0, got {}",
            exploitability
        );
        assert!(
            approx_eq(temporal, 6.4),
            "temporal expected 6.4, got {}",
            temporal
        );
        assert!(
            approx_eq(adjusted_impact, 10.0),
            "adjusted impact expected 10.0, got {}",
            adjusted_impact
        );
        assert!(
            approx_eq(environmental, 9.1),
            "environmental expected 9.1, got {}",
            environmental
        );
    }

    #[test]
    fn spec_cve_2003_0818() {
        // See https://nvd.nist.gov/vuln-metrics/cvss/v2-calculator?vector=(AV:N/AC:L/Au:N/C:C/I:C/A:C/E:F/RL:OF/RC:C/CDP:H/TD:H/CR:M/IR:M/AR:L)
        let v = "AV:N/AC:L/Au:N/C:C/I:C/A:C/E:F/RL:OF/RC:C/CDP:H/TD:H/CR:M/IR:M/AR:L";
        let cvss = Vector::from_str(v).expect("parse vector");

        let base = cvss.base_score().value();
        let impact = cvss.impact().roundup().value();
        let exploitability = cvss.exploitability().roundup().value();
        let temporal = cvss.temporal_score().value();
        let environmental = cvss.environmental_score().value();
        let adjusted_impact = cvss.adjusted_impact().roundup().value();

        assert!(
            approx_eq(base, 10.0),
            "base score expected 10.0, got {}",
            base
        );
        assert!(
            approx_eq(impact, 10.0),
            "impact expected 10.0, got {}",
            impact
        );
        assert!(
            approx_eq(exploitability, 10.0),
            "exploitability expected 10.0, got {}",
            exploitability
        );
        assert!(
            approx_eq(temporal, 8.3),
            "temporal expected 8.3, got {}",
            temporal
        );
        assert!(
            approx_eq(adjusted_impact, 9.6),
            "adjusted impact expected 9.6, got {}",
            adjusted_impact
        );
        assert!(
            approx_eq(environmental, 9.0),
            "environmental expected 9.0, got {}",
            environmental
        );
    }
}
