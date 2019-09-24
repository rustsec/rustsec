//! Warnings sourced from the Advisory DB

use crate::{advisory, package::Package};
use serde::{Deserialize, Serialize};

/// Warnings sourced from the Advisory DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Warning {
    /// Security advisory warning was sourced from
    pub advisory: advisory::Metadata,

    /// Versions impacted by this warning
    pub versions: advisory::Versions,

    /// Name of the dependent package
    pub package: Package,
}

impl Warning {
    /// Create `Warning` about a given [`Advisory`] and [`Package`]
    pub(crate) fn new(
        advisory: &advisory::Metadata,
        versions: &advisory::Versions,
        package: &Package,
    ) -> Self {
        Self {
            advisory: advisory.clone(),
            versions: versions.clone(),
            package: package.clone(),
        }
    }
}
