//! Database containing `RustSec` security advisories

mod entries;
mod index;
mod query;

pub use self::query::Query;

use self::{entries::Entries, index::Index};
use crate::{
    advisory::{self, Advisory},
    collection::Collection,
    error::Error,
    fs,
    vulnerability::Vulnerability,
    Lockfile,
};
use std::path::{Path, PathBuf};

/// Directory under `~/.cargo` where the advisory-db repo will be kept
const ADVISORY_DB_DIRECTORY: &str = "advisory-db";

/// The default URL from which the DB is downloaded
const DEFAULT_DB_URL: &str = "https://github.com/zip-rs/zip/archive/refs/heads/master.zip";

#[cfg(feature = "git")]
use crate::repository::git;

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
}

impl Database {
    /// Open [`Database`] located at the given local path
    pub fn open(path: &Path) -> Result<Self, Error> {
        let mut advisory_paths = vec![];

        for collection in Collection::all() {
            let collection_path = path.join(collection.as_str());

            if let Ok(collection_entry) = fs::read_dir(&collection_path) {
                for dir_entry in collection_entry {
                    let dir_entry = dir_entry?;
                    if !dir_entry.file_type()?.is_dir() {
                        continue;
                    }
                    for advisory_entry in fs::read_dir(dir_entry.path())? {
                        let advisory_path = advisory_entry?.path();
                        let file_name = advisory_path.file_name().and_then(|f| f.to_str());
                        // skip dotfiles like .DS_Store
                        if file_name.map_or(false, |f| f.starts_with('.')) {
                            continue;
                        }
                        advisory_paths.push(advisory_path);
                    }
                }
            }
        }

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
        })
    }

    /// Load [`Database`] from the given directory
    pub fn load_from_dir(path: &Path) -> Result<Self, Error> {
        let db = Self::open(path)?;
        Ok(db)
    }

    /// Fetch the default advisory database from GitHub
    pub fn fetch() -> Result<Self, Error> {
        let local_dir = Database::default_path();
        crate::fetch::fetch(DEFAULT_DB_URL, &local_dir)?;
        Self::load_from_dir(&local_dir)
    }

    /// Look up an advisory by an advisory ID (e.g. "RUSTSEC-YYYY-XXXX")
    pub fn get(&self, id: &advisory::Id) -> Option<&Advisory> {
        self.advisories.find_by_id(id)
    }

    /// Query the database according to the given query object
    pub fn query(&self, query: &Query) -> Vec<&Advisory> {
        // Use indexes if we know a package name and collection
        if let Some(name) = &query.package_name {
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
    pub fn query_vulnerabilities(&self, lockfile: &Lockfile, query: &Query) -> Vec<Vulnerability> {
        let mut vulns = vec![];

        for package in &lockfile.packages {
            let advisories = self.query(&query.clone().package(package));

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
        self.query_vulnerabilities(lockfile, &Query::crate_scope())
    }

    /// Iterate over all of the advisories in the database
    pub fn iter(&self) -> Iter<'_> {
        self.advisories.iter()
    }

    /// Location of the default `advisory-db` repository for crates.io
    pub fn default_path() -> PathBuf {
        home::cargo_home()
            .unwrap_or_else(|err| {
                panic!("Error locating Cargo home directory: {}", err);
            })
            .join(ADVISORY_DB_DIRECTORY)
    }
}

impl IntoIterator for Database {
    type Item = Advisory;

    type IntoIter = std::vec::IntoIter<Advisory>;

    fn into_iter(self) -> Self::IntoIter {
        self.advisories.into_iter()
    }
}
