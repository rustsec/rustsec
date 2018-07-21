//! Parser for `Cargo.lock` files

use std::{fs::File, io::Read, path::Path};
use toml;

use db::AdvisoryDatabase;
use error::Error;
use package::Package;
use vulnerability::Vulnerability;

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

    /// Find all relevant vulnerabilities for this lockfile using the given database
    pub fn vulnerabilities(&self, db: &AdvisoryDatabase) -> Vec<Vulnerability> {
        Vulnerability::find_all(db, self)
    }
}

#[cfg(test)]
mod tests {
    use lockfile::Lockfile;

    #[test]
    fn load_cargo_lockfile() {
        let lockfile = Lockfile::load("Cargo.lock").unwrap();
        assert!(lockfile.packages.len() > 0);
    }
}
