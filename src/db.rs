//! Database containing `RustSec` security advisories

mod entries;
mod index;
mod iter;

pub use self::iter::Iter;

use self::{entries::Entries, index::Index};
use crate::{
    advisory::{self, Advisory},
    error::Error,
    lockfile::Lockfile,
    package,
    repository::Repository,
    version::Version,
    vulnerability,
};

/// Database of RustSec security advisories, indexed both by ID and crate
#[derive(Debug)]
pub struct Database {
    /// All advisories in the database
    advisories: Entries,

    /// Index of Rust core vulnerabilities
    rust_index: Index,

    /// Index of third party crates
    crate_index: Index,
}

impl Database {
    /// Fetch the default advisory database from GitHub
    #[cfg(feature = "chrono")]
    pub fn fetch() -> Result<Self, Error> {
        let repo = Repository::fetch_default_repo()?;
        Self::from_repository(&repo)
    }

    /// Create a new `Database` from the given [`Repository`]
    pub fn from_repository(repo: &Repository) -> Result<Self, Error> {
        let advisory_paths = repo.advisories()?;

        let mut advisories = Entries::new();
        let mut rust_index = Index::new();
        let mut crate_index = Index::new();

        for path in &advisory_paths {
            if let Some(advisory) = advisories.load_file(path)? {
                match advisory.metadata.collection.unwrap() {
                    package::Collection::Crates => {
                        crate_index.insert(&advisory.metadata.package, &advisory.metadata.id);
                    }
                    package::Collection::Rust => {
                        rust_index.insert(&advisory.metadata.package, &advisory.metadata.id);
                    }
                }
            }
        }

        Ok(Self {
            advisories,
            crate_index,
            rust_index,
        })
    }

    /// Look up an advisory by an advisory ID (e.g. "RUSTSEC-YYYY-XXXX")
    pub fn find(&self, id: &advisory::Id) -> Option<&Advisory> {
        self.advisories.get(id)
    }

    /// Look up advisories relevant to a particular crate
    pub fn find_by_crate<N: AsRef<package::Name>>(&self, crate_name: N) -> Vec<&Advisory> {
        if let Some(ids) = self.crate_index.get(crate_name.as_ref()) {
            ids.map(|id| self.find(&id).unwrap()).collect()
        } else {
            vec![]
        }
    }

    /// Find advisories that are unpatched and impact a given crate and version
    pub fn advisories_for_crate<N: AsRef<package::Name>>(
        &self,
        crate_name: N,
        version: &Version,
    ) -> Vec<&Advisory> {
        self.find_by_crate(crate_name)
            .iter()
            .filter(|advisory| {
                let patched_or_unaffected =
                    [&advisory.versions.patched, &advisory.versions.unaffected]
                        .iter()
                        .any(|versions| versions.iter().any(|req| req.matches(version)));

                !patched_or_unaffected
            })
            .cloned()
            .collect()
    }

    /// Return a collection of vulnerabilities for the given lockfile
    pub fn vulnerabilities(&self, lockfile: &Lockfile) -> vulnerability::Collection {
        vulnerability::Collection::find(self, lockfile)
    }

    /// Iterate over all of the advisories in the database
    pub fn iter(&self) -> Iter<'_> {
        self.advisories.iter()
    }
}
