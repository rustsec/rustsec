//! Parser for `Cargo.lock` files

#[cfg(feature = "petgraph")]
pub mod dependency_graph;

use crate::{
    database::{Database, Query},
    error::{Error, ErrorKind},
    package::Package,
    vulnerability::Vulnerability,
};
use serde::Deserialize;
use std::{fs, path::Path, str::FromStr};
use toml;

#[cfg(feature = "petgraph")]
pub use self::dependency_graph::DependencyGraph;

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

        let lockfile: Lockfile = fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?
            .parse()?;

        if lockfile.packages.is_empty() {
            fail!(
                ErrorKind::Parse,
                "no [package] entries found in {}",
                path.display()
            );
        }

        Ok(lockfile)
    }

    /// Find vulnerabilities for the current lockfile
    pub fn vulnerabilities(&self, db: &Database) -> Vec<Vulnerability> {
        self.query_vulnerabilities(db, &Query::crate_scope())
    }

    /// Find vulnerabilities for the current lockfile which match a given query
    pub fn query_vulnerabilities(&self, db: &Database, query: &Query) -> Vec<Vulnerability> {
        let mut vulns = vec![];

        for package in &self.packages {
            let advisories = db.query(
                &query
                    .clone()
                    .package_version(package.name.clone(), package.version.clone()),
            );

            vulns.extend(
                advisories
                    .iter()
                    .map(|advisory| Vulnerability::new(advisory, package)),
            );
        }

        vulns
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

    /// Get the dependency graph for this lockfile
    #[cfg(feature = "petgraph")]
    pub fn dependency_graph(&self) -> DependencyGraph {
        DependencyGraph::new(self)
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
