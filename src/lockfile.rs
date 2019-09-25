//! Parser for `Cargo.lock` files

#[cfg(feature = "dependency-tree")]
use crate::dependency::Tree;
use crate::{
    error::{Error, ErrorKind},
    metadata::Metadata,
    package::Package,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, str::FromStr, string::ToString};
use toml;

/// Parsed Cargo.lock file containing dependencies
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Lockfile {
    /// Dependencies enumerated in the lockfile
    #[serde(rename = "package")]
    pub packages: Vec<Package>,

    /// Package metadata
    #[serde(default)]
    pub metadata: Metadata,
}

impl Lockfile {
    /// Load lock data from a `Cargo.lock` file
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        match fs::read_to_string(path.as_ref()) {
            Ok(s) => s.parse(),
            Err(e) => fail!(
                ErrorKind::Io,
                "couldn't open {}: {}",
                path.as_ref().display(),
                e
            ),
        }
    }

    /// Enumerate dependent [`Package`] types for the given parent [`Package`].
    pub fn dependent_packages(&self, package: &Package) -> Vec<&Package> {
        let mut result = vec![];

        for dependency in &package.dependencies {
            result.push(
                self.packages
                    .iter()
                    .find(|pkg| dependency.matches(pkg))
                    .unwrap(),
            )
        }

        result
    }

    /// Get the dependency tree for this `Lockfile`. Returns an error if the
    /// contents of this lockfile aren't well structured.
    ///
    /// The `dependency-tree` Cargo feature must be enabled to use this.
    #[cfg(feature = "dependency-tree")]
    pub fn dependency_tree(&self) -> Result<Tree, Error> {
        Tree::new(self)
    }
}

impl FromStr for Lockfile {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        Ok(toml::from_str(toml_string)?)
    }
}

impl ToString for Lockfile {
    fn to_string(&self) -> String {
        toml::to_string(self).unwrap()
    }
}
