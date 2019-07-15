//! Parser for `Cargo.lock` files

use crate::{error::Error, package::Package};
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};
use toml;

/// Parsed Cargo.lock file containing dependencies
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Lockfile {
    /// Dependencies enumerated in the lockfile
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

impl Lockfile {
    /// Load lock data from a `Cargo.lock` file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(path.as_ref())?;
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        Self::from_toml(&toml)
    }

    /// Parse the TOML data from the `Cargo.lock` file
    pub fn from_toml(toml_string: &str) -> Result<Self, Error> {
        Ok(toml::from_str(toml_string)?)
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
