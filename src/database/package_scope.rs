use serde::{Deserialize, Serialize};

/// Scope for querying which kind of packages should be considered.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PackageScope {
    /// `LocalCrates` are crates without any source e.g. crates linked in a workspace.
    LocalCrates,

    /// `PublicCrates` are the the crate with a source e.g. crates from `crates.io`.
    PublicCrates,

    /// `All` package types should considered.
    All
}

impl PackageScope {
    /// Determines if scope is only local crates
    pub fn is_local(&self) -> bool {
        self == &PackageScope::LocalCrates
    }
}

impl Default for PackageScope {
    fn default() -> Self {
        PackageScope::All
    }
}