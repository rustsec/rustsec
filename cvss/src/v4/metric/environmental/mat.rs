//! Attack Requirements (MAT)

use crate::{
    v4::metric::{Metric, MetricType},
    Error, Result,
};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Attack Requirements (MAT) - CVSS v4.0 Environmental Metric Group
///
/// Described in CVSS v4.0 Specification: Section 4.2
///
/// > This metric captures the prerequisite **deployment and execution
/// > conditions or variables** of the vulnerable system that enable the attack.
/// > These differ from security-enhancing techniques/technologies (ref _Attack
/// > Complexity_) as the primary purpose of these conditions is **not** to
/// > explicitly mitigate attacks, but rather, emerge naturally as a consequence
/// > of the deployment and execution of the vulnerable system. If the attacker
/// > does not take action to overcome these conditions, the attack may succeed
/// > only occasionally or not succeed at all.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ModifiedAttackRequirements {
    /// Not Defined (X)
    ///
    /// > The metric has not been evaluated.
    NotDefined,

    /// Present (P)
    ///
    /// > The successful attack depends on the presence of specific deployment
    /// > and execution conditions of the vulnerable system that enable the
    /// > attack. These include: A **race condition** must be won to
    /// > successfully exploit the vulnerability. The successfulness of the
    /// > attack is conditioned on execution conditions that are not under full
    /// > control of the attacker. The attack may need to be launched multiple
    /// > times against a single target before being successful. Network
    /// > injection. The attacker must inject themselves into the logical
    /// > network path between the target and the resource requested by the
    /// > victim (e.g. vulnerabilities requiring an on-path attacker).
    Present,

    /// None (N)
    ///
    /// > The successful attack does not depend on the deployment and execution
    /// > conditions of the vulnerable system. The attacker can expect to be
    /// > able to reach the vulnerability and execute the exploit under all or
    /// > most instances of the vulnerability.
    None,
}

impl Default for ModifiedAttackRequirements {
    fn default() -> Self {
        Self::NotDefined
    }
}

impl Metric for ModifiedAttackRequirements {
    const TYPE: MetricType = MetricType::MAT;

    fn as_str(self) -> &'static str {
        match self {
            ModifiedAttackRequirements::NotDefined => "X",
            ModifiedAttackRequirements::Present => "P",
            ModifiedAttackRequirements::None => "N",
        }
    }
}

impl fmt::Display for ModifiedAttackRequirements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ModifiedAttackRequirements {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(ModifiedAttackRequirements::NotDefined),
            "P" => Ok(ModifiedAttackRequirements::Present),
            "N" => Ok(ModifiedAttackRequirements::None),
            _ => Err(Error::InvalidMetricV4 {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}