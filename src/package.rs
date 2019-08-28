//! Crate metadata as parsed from `Cargo.lock`

use crate::version::Version;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A Rust package (i.e. crate) as structured in `Cargo.lock`
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Package {
    /// Name of a crate
    pub name: Name,

    /// Crate version (using `semver`)
    pub version: Version,

    /// Source of the crate
    pub source: Option<String>,

    /// Dependencies of this crate
    #[serde(default)]
    pub dependencies: Vec<String>,
}

/// Name of a Rust package
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Name(pub String);

impl Name {
    /// Get string reference to this package name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<Name> for Name {
    fn as_ref(&self) -> &Name {
        self
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<&'a str> for Name {
    fn from(string: &'a str) -> Name {
        Name(string.into())
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}
