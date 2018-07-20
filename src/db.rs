//! Database containing `RustSec` security advisories

use reqwest;
use semver::Version;
use std::{
    collections::{
        btree_map::{Entry, Iter as BTMapIter},
        BTreeMap,
    },
    io::Read,
    str,
};
use toml;

use advisory::{Advisory, AdvisoryId};
use error::{Error, ErrorKind};
use lockfile::Lockfile;
use package::PackageName;
use vulnerability::Vulnerability;
use ADVISORY_DB_URL;

/// A collection of security advisories, indexed both by ID and crate
#[derive(Debug)]
pub struct AdvisoryDatabase {
    advisories: BTreeMap<AdvisoryId, Advisory>,
    crates: BTreeMap<PackageName, Vec<AdvisoryId>>,
}

#[derive(Deserialize)]
struct AdvisoryList {
    #[serde(rename = "advisory")]
    advisories: Vec<Advisory>,
}

impl AdvisoryDatabase {
    /// Fetch the advisory database from the server where it is stored
    pub fn fetch() -> Result<Self, Error> {
        Self::fetch_from_url(ADVISORY_DB_URL)
    }

    /// Fetch advisory database from a custom URL
    pub fn fetch_from_url(url: &str) -> Result<Self, Error> {
        let mut response = reqwest::get(url)?;

        if !response.status().is_success() {
            fail!(
                ErrorKind::ServerResponse,
                "bad response status: {}",
                response.status()
            );
        }

        let mut body = Vec::new();
        response.read_to_end(&mut body)?;
        let response_str = str::from_utf8(&body)?;

        Self::from_toml(response_str)
    }

    /// Parse the advisory database from a TOML serialization of it
    pub fn from_toml(toml_string: &str) -> Result<Self, Error> {
        let advisory_list: AdvisoryList = toml::from_str(toml_string)?;

        let mut advisories = BTreeMap::new();
        let mut crates = BTreeMap::new();

        for advisory in &advisory_list.advisories {
            let mut crate_advisories = match crates.entry(advisory.package.clone()) {
                Entry::Vacant(entry) => entry.insert(vec![]),
                Entry::Occupied(entry) => entry.into_mut(),
            };

            crate_advisories.push(advisory.id.clone());
            advisories.insert(advisory.id.clone(), advisory.clone());
        }

        Ok(Self { advisories, crates })
    }

    /// Look up an advisory by an advisory ID (e.g. "RUSTSEC-YYYY-XXXX")
    pub fn find<I: Into<AdvisoryId>>(&self, id: I) -> Option<&Advisory> {
        self.advisories.get(&id.into())
    }

    /// Look up advisories relevant to a particular crate
    pub fn find_by_crate<N: Into<PackageName>>(&self, crate_name: N) -> Vec<&Advisory> {
        if let Some(ids) = self.crates.get(&crate_name.into()) {
            ids.iter()
                .map(|id| self.find(id.clone()).unwrap())
                .collect()
        } else {
            vec![]
        }
    }

    /// Find advisories that are unpatched and impact a given crate and version
    pub fn advisories_for_crate<N: Into<PackageName>>(
        &self,
        crate_name: N,
        version: &Version,
    ) -> Vec<&Advisory> {
        let mut results = self.find_by_crate(crate_name);
        results.retain(|advisory| {
            !advisory
                .patched_versions
                .iter()
                .any(|req| req.matches(version))
        });
        results
    }

    /// Find all vulnerabilities for a given `AdvisoryDatabase` and `Lockfile`
    pub fn vulns_for_lockfile(&self, lockfile: &Lockfile) -> Vec<Vulnerability> {
        let mut result = Vec::new();

        for package in &lockfile.packages {
            for advisory in self.advisories_for_crate(package.name.clone(), &package.version) {
                result.push(Vulnerability::new(advisory, &package))
            }
        }

        result
    }

    /// Iterate over all of the advisories in the database
    pub fn iter(&self) -> Iter {
        Iter(self.advisories.iter())
    }
}

impl<'a> IntoIterator for &'a AdvisoryDatabase {
    type Item = (&'a AdvisoryId, &'a Advisory);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

/// Iterator over the advisory database
pub struct Iter<'a>(BTMapIter<'a, AdvisoryId, Advisory>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a AdvisoryId, &'a Advisory);

    fn next(&mut self) -> Option<(&'a AdvisoryId, &'a Advisory)> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use db::AdvisoryDatabase;
    use semver::Version;

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
    fn test_advisories_for_crate() {
        let db = example_advisory_db();
        let version = Version::parse(EXAMPLE_VERSION).unwrap();
        let advisories = db.advisories_for_crate(EXAMPLE_PACKAGE, &version);

        assert_eq!(advisories[0], db.find(EXAMPLE_ADVISORY).unwrap());
    }
}
