//! Parser for `Cargo.lock` files

use crate::{
    error::{Error, ErrorKind},
    metadata::Metadata,
    package::Package,
};
use serde::Deserialize;
use std::{fs, path::Path, str::FromStr};
use toml;

/// Parsed Cargo.lock file containing dependencies
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?
            .parse()
    }

    /// Find dependencies for the given package
    pub fn dependencies(&self, package: &Package) -> Vec<&Package> {
        let mut result = vec![];

        for dependency in &package.dependencies {
            result.push(
                self.packages
                    .iter()
                    .find(|pkg| dependency.name == pkg.name && dependency.version == pkg.version)
                    .unwrap(),
            )
        }

        result
    }
}

impl FromStr for Lockfile {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        let lockfile: Self = toml::from_str(toml_string)?;

        if lockfile.packages.is_empty() {
            fail!(ErrorKind::Parse, "no [package] entries found");
        }

        Ok(lockfile)
    }
}

#[cfg(test)]
mod tests {
    use crate::lockfile::Lockfile;

    #[test]
    fn load_cargo_lockfile() {
        let lockfile = Lockfile::load("Cargo.lock").unwrap();
        assert!(lockfile.packages.len() > 0);
    }
}
