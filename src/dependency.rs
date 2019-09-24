//! Package dependencies

#[cfg(feature = "dependency-tree")]
pub mod graph;
#[cfg(feature = "dependency-tree")]
mod tree;

#[cfg(feature = "dependency-tree")]
pub use self::tree::Tree;

use crate::{
    error::{Error, ErrorKind},
    package::{Name, Package, Source},
};
use semver::Version;
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Package dependencies
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Dependency {
    /// Name of the dependency
    pub name: Name,

    /// Version of the dependency
    pub version: Version,

    /// Source for the dependency
    pub source: Option<Source>,
}

impl Dependency {
    /// Does the given [`Package`] satisfy this dependency?
    pub fn matches(&self, package: &Package) -> bool {
        self.name == package.name
            && self.version == package.version
            && self.source == package.source
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.name, &self.version)?;

        if let Some(source) = &self.source {
            write!(f, " ({})", source)?;
        }

        Ok(())
    }
}

impl FromStr for Dependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut parts = s.split_whitespace();

        let name = parts
            .next()
            .ok_or_else(|| format_err!(ErrorKind::Parse, "empty dependency string"))?
            .parse()?;

        let version = parts
            .next()
            .ok_or_else(|| format_err!(ErrorKind::Parse, "missing version for dependency: {}", s))?
            .parse()?;

        let source = parts
            .next()
            .map(|s| {
                if s.len() < 2 || !s.starts_with('(') || !s.ends_with(')') {
                    Err(format_err!(
                        ErrorKind::Parse,
                        "malformed source in dependency: {}",
                        s
                    ))
                } else {
                    s[1..(s.len() - 1)].parse()
                }
            })
            .transpose()?;

        if parts.next().is_some() {
            fail!(ErrorKind::Parse, "malformed dependency: {}", s);
        }

        Ok(Self {
            name,
            version,
            source,
        })
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        String::deserialize(deserializer)
            .and_then(|ref s| Self::from_str(s).map_err(D::Error::custom))
    }
}

impl Serialize for Dependency {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
