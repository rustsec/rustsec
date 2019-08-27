//! Crate metadata as parsed from `Cargo.lock`

use crate::version::Version;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A Rust package (i.e. crate) as structured in `Cargo.lock`
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Package {
    /// Name of a crate
    pub name: PackageName,

    /// Crate version (using `semver`)
    pub version: Version,

    /// Source of the crate
    pub source: Option<String>,

    /// Dependencies of this crate
    pub dependencies: Option<Vec<String>>,
}

/// Name of a crate
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct PackageName(pub String);

impl PackageName {
    /// Get string reference to this package name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<PackageName> for PackageName {
    fn as_ref(&self) -> &PackageName {
        self
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<&'a str> for PackageName {
    fn from(string: &'a str) -> PackageName {
        PackageName(string.into())
    }
}

impl Into<String> for PackageName {
    fn into(self) -> String {
        self.0
    }
}
