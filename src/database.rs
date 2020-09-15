//! Database containing `RustSec` security advisories

mod entries;
mod index;
mod query;

pub mod scope;

pub use self::query::Query;

use self::{entries::Entries, index::Index};
use crate::{
    advisory::{self, Advisory},
    collection::Collection,
    error::Error,
    lockfile::Lockfile,
    repository::{Commit, Repository},
    vulnerability::Vulnerability,
};

/// Iterator over entries in the database
pub type Iter<'a> = std::slice::Iter<'a, Advisory>;

/// Database of RustSec security advisories, indexed both by ID and collection
#[derive(Debug)]
pub struct Database {
    /// All advisories in the database
    advisories: Entries,

    /// Index of Rust core vulnerabilities
    rust_index: Index,

    /// Index of third party crates
    crate_index: Index,

    /// Information about the last git commit to the database
    latest_commit: Commit,
}

impl Database {
    /// Load [`Database`] from the given [`Repository`]
    pub fn load(repo: &impl Repository) -> Result<Self, Error> {
        let advisory_paths = repo.advisories()?;
        let latest_commit = repo.latest_commit()?;

        let mut advisories = Entries::new();
        let mut rust_index = Index::new();
        let mut crate_index = Index::new();

        for path in &advisory_paths {
            if let Some(slot) = advisories.load_file(path)? {
                let advisory = advisories.get(slot).unwrap();
                match advisory.metadata.collection.unwrap() {
                    Collection::Crates => {
                        crate_index.insert(&advisory.metadata.package, slot);
                    }
                    Collection::Rust => {
                        rust_index.insert(&advisory.metadata.package, slot);
                    }
                }
            }
        }

        Ok(Self {
            advisories,
            crate_index,
            rust_index,
            latest_commit,
        })
    }

    /// Look up an advisory by an advisory ID (e.g. "RUSTSEC-YYYY-XXXX")
    pub fn get(&self, id: &advisory::Id) -> Option<&Advisory> {
        self.advisories.find_by_id(id)
    }

    /// Query the database according to the given query object
    pub fn query(&self, query: &Query) -> Vec<&Advisory> {
        // Use indexes if we know a package name and collection
        if let Some(name) = &query.package {
            if let Some(collection) = query.collection {
                return match collection {
                    Collection::Crates => self.crate_index.get(name),
                    Collection::Rust => self.rust_index.get(name),
                }
                .map(|slots| {
                    slots
                        .map(|slot| self.advisories.get(*slot).unwrap())
                        .filter(|advisory| query.matches(advisory))
                        .collect()
                })
                .unwrap_or_else(Vec::new);
            }
        }

        self.iter()
            .filter(|advisory| query.matches(advisory))
            .collect()
    }

    /// Find vulnerabilities in the provided `Lockfile` which match a given query.
    // TODO(tarcieri): move `package_scope` into `Query`?
    pub fn query_vulnerabilities(
        &self,
        lockfile: &Lockfile,
        query: &Query,
        package_scope: impl Into<scope::Package>,
    ) -> Vec<Vulnerability> {
        let package_scope = package_scope.into();
        let mut vulns = vec![];

        for package in &lockfile.packages {
            if package_scope.is_remote() && package.source.is_none() {
                continue;
            }

            let advisories = self.query(
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

    /// Scan for vulnerabilities in the provided `Lockfile`.
    pub fn vulnerabilities(&self, lockfile: &Lockfile) -> Vec<Vulnerability> {
        self.query_vulnerabilities(lockfile, &Query::crate_scope(), scope::Package::default())
    }

    /// Iterate over all of the advisories in the database
    pub fn iter(&self) -> Iter<'_> {
        self.advisories.iter()
    }

    /// Get information about the latest commit to the repo
    pub fn latest_commit(&self) -> &Commit {
        &self.latest_commit
    }
}
