//! Parser for `Cargo.lock` files

#[cfg(feature = "dependency-tree")]
use crate::dependency::Tree;
use crate::{
    error::{Error, ErrorKind},
    metadata::Metadata,
    package::Package,
    Map,
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

    /// Get the root [`Package`] in this `Lockfile`
    pub fn root_package(&self) -> &Package {
        // We assert a valid root exists at the time the `Lockfile` is parsed
        // inside of `FromStr` so it's safe to unwrap here
        find_root_package(self).unwrap()
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
        let lockfile: Self = toml::from_str(toml_string)?;

        if lockfile.packages.is_empty() {
            fail!(ErrorKind::Parse, "no [package] entries found");
        }

        // Ensure the lockfile has a valid root package
        find_root_package(&lockfile)?;

        Ok(lockfile)
    }
}

impl ToString for Lockfile {
    fn to_string(&self) -> String {
        toml::to_string(self).unwrap()
    }
}

/// Find the root package in the given lockfile
fn find_root_package(lockfile: &Lockfile) -> Result<&Package, Error> {
    let mut dependency_counts = Map::new();

    for package in &lockfile.packages {
        dependency_counts.entry(&package.name).or_insert(0);

        for dependency in &package.dependencies {
            *dependency_counts.entry(&dependency.name).or_insert(0) += 1;
        }
    }

    let root_package_name = *dependency_counts
        .iter()
        .find(|(_, count)| **count == 0)
        .ok_or_else(|| format_err!(ErrorKind::Parse, "couldn't find root package"))?
        .0;

    Ok(lockfile
        .packages
        .iter()
        .find(|package| &package.name == root_package_name)
        .unwrap())
}
