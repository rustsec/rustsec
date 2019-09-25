//! Rust packages enumerated in `Cargo.lock`

pub use semver::Version;

use crate::{dependency::Dependency, error::Error};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Information about a Rust package (as sourced from `Cargo.lock`)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Package {
    /// Name of the package
    pub name: Name,

    /// Version of the package
    pub version: Version,

    /// Source for the package
    pub source: Option<Source>,

    /// Dependencies of the package
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

impl From<Package> for Dependency {
    /// Get the [`Dependency`] requirement for this `[[package]]`
    fn from(pkg: Package) -> Dependency {
        Self {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
            source: pkg.source.clone(),
        }
    }
}

/// Name of a Rust `[[package]]`
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Name(String);

impl Name {
    /// Get package name as an `&str`
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        // TODO(tarcieri): ensure name is valid
        Ok(Name(s.into()))
    }
}

/// Source for a Rust `[[package]]`
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Source(String);

impl Source {
    /// Get source as an `&str`
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Source {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        // TODO(tarcieri): ensure source is valid
        Ok(Source(s.into()))
    }
}
