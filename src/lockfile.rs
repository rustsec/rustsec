//! Parser for `Cargo.lock` files

use crate::{
    error::{Error, ErrorKind},
    package::Package,
};
use serde::Deserialize;
use std::{fs, path::Path, str::FromStr};
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
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?
            .parse()
    }
}

impl FromStr for Lockfile {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        Ok(toml::from_str(toml_string)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::lockfile::Lockfile;

    #[test]
    fn load_cargo_lockfile() {
        let lockfile = Lockfile::load_file("Cargo.lock").unwrap();
        assert!(lockfile.packages.len() > 0);
    }
}
