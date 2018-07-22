//! Crate metadata as parsed from `Cargo.lock`

use semver::Version;
use std::fmt;

/// A Rust package (i.e. crate) as structured in `Cargo.lock`
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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

impl AsRef<str> for PackageName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
