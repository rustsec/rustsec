use serde::{Deserialize, Serialize};

/// Source of a package
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PackageSource {
    /// Package is local
    #[serde(rename = "local")]
    Local,

    /// Package is located in a specific registry with `String` uri
    #[serde(rename = "registry")]
    Registry(String),

    /// Package is located somewhere public
    #[serde(rename = "public")]
    Public,

    /// All sources should be considered
    #[serde(rename = "all")]
    All,
}

/// Scope for querying which kind of packages should be considered.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageScope {
    /// Source of a package
    pub source: Vec<PackageSource>,
}

impl PackageScope {
    /// Determines if scope is only local crates
    pub fn is_no_local(&self) -> bool {
        if self.source.contains(&PackageSource::Public) {
            return true;
        }
        for source in &self.source {
            if let PackageSource::Registry(_some) = &source {
                return true;
            }
        }

        false
    }
}

impl Default for PackageScope {
    fn default() -> Self {
        Self::from_source(PackageSource::All)
    }
}

impl PackageScope {
    /// Creates a new [[PackageScope]] from a specific registry uri `source`
    pub fn from_registry(source: &str) -> Self {
        Self::from_source(PackageSource::Registry(source.to_string()))
    }

    /// Creates a new [[PackageScope]] with a specific [[PackageSource]]
    pub fn from_source(source: PackageSource) -> Self {
        PackageScope {
            source: vec![source],
        }
    }
}
