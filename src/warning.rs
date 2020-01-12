//! Warnings sourced from the Advisory DB

use crate::{advisory, package::Package};
use serde::{Deserialize, Serialize};

/// Warnings sourced from the Advisory DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Warning {
    /// Kind of warning
    pub kind: Kind,

    /// Name of the dependent package
    pub package: Package,
}

impl Warning {
    /// Create `Warning` of the given kind
    pub(crate) fn new(kind: Kind, package: &Package) -> Self {
        Self {
            kind,
            package: package.clone(),
        }
    }
}

/// Kinds of warnings
#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum Kind {
    /// Unmaintained packages
    #[serde(rename = "unmaintained")]
    Unmaintained {
        /// Source advisory
        advisory: advisory::Metadata,

        /// Versions impacted by this warning
        versions: advisory::Versions,
    },

    /// Informational advisories
    #[serde(rename = "informational")]
    Informational {
        /// Source advisory
        advisory: advisory::Metadata,

        /// Versions impacted by this warning
        versions: advisory::Versions,
    },

    /// Yanked packages
    #[serde(rename = "yanked")]
    Yanked,
}
