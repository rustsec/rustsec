//! Database scopes

use serde::{Deserialize, Serialize};

/// Registries where packages are located
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Registry {
    /// Public package published to <https://crates.io>
    #[serde(rename = "public")]
    Public,

    /// Package is local
    #[serde(rename = "local")]
    Local,

    /// Package is located in a private registry
    #[serde(rename = "private")]
    Private {
        /// URI of the private registry
        uri: String,
    },

    /// All sources should be considered
    #[serde(rename = "all")]
    All,
}

impl Default for Registry {
    fn default() -> Self {
        Registry::Public
    }
}

/// Scopes for packages to be queried (i.e. their sources)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    /// Source of a package
    pub source: Vec<Registry>,
}

impl Package {
    /// Is the scope only for remote crates?
    pub fn is_remote(&self) -> bool {
        self.source.iter().any(|source| match source {
            Registry::Public | Registry::Private { .. } => true,
            _ => false,
        })
    }
}

impl Default for Package {
    fn default() -> Self {
        Registry::default().into()
    }
}

impl Package {
    /// Creates a new [[Scope]] from a specific registry uri `source`
    pub fn from_registry(source: &str) -> Self {
        Registry::Private {
            uri: source.to_string(),
        }
        .into()
    }
}

impl From<Registry> for Package {
    fn from(registry: Registry) -> Self {
        Self {
            source: vec![registry],
        }
    }
}
