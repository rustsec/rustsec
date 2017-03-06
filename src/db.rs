//! Database containing `RustSec` security advisories

use ADVISORY_DB_URL;
use advisory::Advisory;
use error::{Error, Result};
use reqwest;
use semver::Version;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::hash_map::Iter;
use std::io::Read;
use std::str;
use toml;

/// A collection of security advisories, indexed both by ID and crate
#[derive(Debug)]
pub struct AdvisoryDatabase {
    advisories: HashMap<String, Advisory>,
    crates: HashMap<String, Vec<String>>,
}

impl AdvisoryDatabase {
    /// Fetch the advisory database from the server where it is stored
    pub fn fetch() -> Result<Self> {
        Self::fetch_from_url(ADVISORY_DB_URL)
    }

    /// Fetch advisory database from a custom URL
    pub fn fetch_from_url(url: &str) -> Result<Self> {
        let mut response = reqwest::get(url).or(Err(Error::IO))?;

        if !response.status().is_success() {
            return Err(Error::ServerResponse);
        }

        let mut body = Vec::new();
        response.read_to_end(&mut body).or(Err(Error::ServerResponse))?;
        let response_str = str::from_utf8(&body).or(Err(Error::Parse))?;

        Self::from_toml(response_str)
    }

    /// Parse the advisory database from a TOML serialization of it
    pub fn from_toml(data: &str) -> Result<Self> {
        let db_toml = data.parse::<toml::Value>().or(Err(Error::Parse))?;

        let advisories_toml = match db_toml {
            toml::Value::Table(ref table) => {
                match *table.get("advisory").ok_or(Error::MissingAttribute)? {
                    toml::Value::Array(ref arr) => arr,
                    _ => return Err(Error::InvalidAttribute),
                }
            }
            _ => return Err(Error::InvalidAttribute),
        };

        let mut advisories = HashMap::new();
        let mut crates = HashMap::<String, Vec<String>>::new();

        for advisory_toml in advisories_toml.iter() {
            let advisory = match *advisory_toml {
                toml::Value::Table(ref table) => Advisory::from_toml_table(table)?,
                _ => return Err(Error::InvalidAttribute),
            };

            let mut crate_vec = match crates.entry(advisory.package.clone()) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };

            crate_vec.push(advisory.id.clone());
            advisories.insert(advisory.id.clone(), advisory);
        }

        Ok(AdvisoryDatabase {
            advisories: advisories,
            crates: crates,
        })
    }

    /// Look up an advisory by an advisory ID (e.g. "RUSTSEC-YYYY-XXXX")
    pub fn find(&self, id: &str) -> Option<&Advisory> {
        self.advisories.get(id)
    }

    /// Look up advisories relevant to a particular crate
    pub fn find_by_crate(&self, crate_name: &str) -> Vec<&Advisory> {
        let ids = self.crates.get(crate_name);
        let mut result = Vec::new();

        if ids.is_some() {
            for id in ids.unwrap() {
                result.push(self.find(id).unwrap())
            }
        }

        result
    }

    /// Find advisories that are unpatched and impact a given crate and version
    pub fn find_vulns_for_crate(&self,
                                crate_name: &str,
                                version_str: &str)
                                -> Result<Vec<&Advisory>> {
        let version = Version::parse(version_str).or(Err(Error::MalformedVersion))?;
        let mut result = Vec::new();

        for advisory in self.find_by_crate(crate_name) {
            if !advisory.patched_versions.iter().any(|req| req.matches(&version)) {
                result.push(advisory);
            }
        }

        Ok(result)
    }

    /// Iterate over all of the advisories in the database
    pub fn iter(&self) -> Iter<String, Advisory> {
        self.advisories.iter()
    }
}

#[cfg(test)]
mod tests {
    use AdvisoryDatabase;

    pub const EXAMPLE_PACKAGE: &'static str = "heffalump";
    pub const EXAMPLE_VERSION: &'static str = "1.0.0";
    pub const EXAMPLE_ADVISORY: &'static str = "RUSTSEC-1234-0001";

    pub const EXAMPLE_ADVISORIES: &'static str = r#"
        [[advisory]]
        id = "RUSTSEC-1234-0001"
        package = "heffalump"
        patched_versions = [">= 1.1.0"]
        date = "2017-01-01"
        title = "Remote code execution vulnerability in heffalump service"
        description = """
        The heffalump service contained a shell escaping vulnerability which could
        be exploited by an attacker to perform arbitrary code execution.

        The issue was corrected by use of proper shell escaping.
        """
    "#;

    fn example_advisory_db() -> AdvisoryDatabase {
        AdvisoryDatabase::from_toml(EXAMPLE_ADVISORIES).unwrap()
    }

    #[test]
    fn test_find_vulns_for_crate() {
        let db = example_advisory_db();
        let advisories = db.find_vulns_for_crate(EXAMPLE_PACKAGE, EXAMPLE_VERSION).unwrap();

        assert_eq!(advisories[0], db.find(EXAMPLE_ADVISORY).unwrap());
    }
}
