//! Attributes of Rust packages

use crate::{
    error::{Error, ErrorKind},
    version::Version,
};
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Information about a Rust package (as sourced from `Cargo.lock`)
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
pub struct Name(String);

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

/// Collections of packages we collect advisories for
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Collection {
    /// Crates published through crates.io
    Crates,

    /// Rust core vulnerabilities
    Rust,
}

impl Collection {
    /// Get a `str` representing the kind of package
    pub fn as_str(&self) -> &str {
        match self {
            Collection::Crates => "crates",
            Collection::Rust => "rust",
        }
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Collection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "crates" => Collection::Crates,
            "rust" => Collection::Rust,
            other => fail!(ErrorKind::Parse, "invalid package type: {}", other),
        })
    }
}

impl<'de> Deserialize<'de> for Collection {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}

impl Serialize for Collection {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::Collection;

    #[test]
    fn parse_crate() {
        let crate_kind = "crates".parse::<Collection>().unwrap();
        assert_eq!(Collection::Crates, crate_kind);
        assert_eq!("crates", crate_kind.as_str());
    }

    #[test]
    fn parse_rust() {
        let rust_kind = "rust".parse::<Collection>().unwrap();
        assert_eq!(Collection::Rust, rust_kind);
        assert_eq!("rust", rust_kind.as_str());
    }

    #[test]
    fn parse_other() {
        assert!("foobar".parse::<Collection>().is_err());
    }
}
