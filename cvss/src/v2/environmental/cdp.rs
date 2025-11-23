//! CVSS v2.0 Environmental Metric - Collateral Damage Potential (CDP)

use crate::Error;
use crate::v2::{Metric, MetricType};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Collateral Damage Potential (CDP) - CVSS v2.0 Environmental Metric
/// 
/// Described in CVSS v2.0 Specification: Section 2.3.1:
/// <https://www.first.org/cvss/v2/guide#2-3-1-Collateral-Damage-Potential-CDP>
///
/// > This metric measures the potential for loss of life or physical assets
/// > through damage or theft of property or equipment.  The metric may also
/// > measure economic loss of productivity or revenue.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum CollateralDamagePotential {
    /// None (N)
    /// > There is no potential for loss of life, physical assets, productivity
    /// > or revenue.
    None,

    /// Low (L)
    /// > A successful exploit of this vulnerability may result in slight
    /// > physical or property damage. Or, there may be a slight loss of revenue
    /// > or productivity to the organization.
    Low,

    /// Low-Medium (LM)
    /// > A successful exploit of this vulnerability may result in moderate
    /// > physical or property damage. Or, there may be a moderate loss of
    /// > revenue or productivity to the organization.
    LowMedium,

    /// Medium-High (MH)
    /// > A successful exploit of this vulnerability may result in significant
    /// > physical or property damage or loss. Or, there may be a significant
    /// > loss of revenue or productivity.
    MediumHigh,

    /// High (H)
    /// > A successful exploit of this vulnerability may result in catastrophic
    /// > physical or property damage and loss. Or, there may be a catastrophic
    /// > loss of revenue or productivity.
    High,

    /// Not Defined (ND)
    /// > Assigning this value to the metric will not influence the score. It is
    /// > a signal to the equation to skip this metric.
    NotDefined,
}

impl Metric for CollateralDamagePotential {
    const TYPE: MetricType = MetricType::CDP;

    fn score(self) -> f64 {
        match self {
            CollateralDamagePotential::None => 0.0,
            CollateralDamagePotential::Low => 0.1,
            CollateralDamagePotential::LowMedium => 0.3,
            CollateralDamagePotential::MediumHigh => 0.4,
            CollateralDamagePotential::High => 0.5,
            CollateralDamagePotential::NotDefined => 0.0,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            CollateralDamagePotential::None => "N",
            CollateralDamagePotential::Low => "L",
            CollateralDamagePotential::LowMedium => "LM",
            CollateralDamagePotential::MediumHigh => "MH",
            CollateralDamagePotential::High => "H",
            CollateralDamagePotential::NotDefined => "ND",
        }
    }
}

impl fmt::Display for CollateralDamagePotential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for CollateralDamagePotential {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "N" => Ok(CollateralDamagePotential::None),
            "L" => Ok(CollateralDamagePotential::Low),
            "LM" => Ok(CollateralDamagePotential::LowMedium),
            "MH" => Ok(CollateralDamagePotential::MediumHigh),
            "H" => Ok(CollateralDamagePotential::High),
            "ND" => Ok(CollateralDamagePotential::NotDefined),
            _ => Err(Error::InvalidMetricV2 {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
