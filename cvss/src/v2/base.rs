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
use crate::{Error, Metric, MetricType, PREFIX, Result};
use alloc::{borrow::ToOwned, vec::Vec};
use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use {
    alloc::string::{String, ToString},
    serde::{Deserialize, Serialize, de, ser},
};

#[cfg(feature = "std")]
use crate::Severity;

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
