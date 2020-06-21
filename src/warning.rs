//! Warnings sourced from the Advisory DB

use crate::error::{Error, ErrorKind};
use crate::{advisory, package::Package};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Warnings sourced from the Advisory DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Warning {
    /// Kind of warning
    pub kind: Kind,

    /// Name of the dependent package
    pub package: Package,

    /// Source advisory
    pub advisory: Option<advisory::Metadata>,

    /// Versions impacted by this warning
    pub versions: Option<advisory::Versions>,
}

impl Warning {
    /// Create `Warning` of the given kind
    pub fn new(
        kind: Kind,
        package: &Package,
        advisory: Option<advisory::Metadata>,
        versions: Option<advisory::Versions>,
    ) -> Self {
        Self {
            kind,
            package: package.clone(),
            advisory,
            versions,
        }
    }
}

/// Kinds of warnings
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum Kind {
    /// Unmaintained packages
    #[serde(rename = "unmaintained")]
    Unmaintained,

    /// Unsound packages
    #[serde(rename = "unsound")]
    Unsound,

    /// Informational advisories
    #[serde(rename = "informational")]
    Informational,

    /// Yanked packages
    #[serde(rename = "yanked")]
    Yanked,
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "unmaintained" => Kind::Unmaintained,
            "informational" => Kind::Informational,
            "yanked" => Kind::Yanked,
            other => fail!(ErrorKind::Parse, "invalid warning type: {}", other),
        })
    }
}
