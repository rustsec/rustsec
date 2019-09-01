//! Database containing `RustSec` security advisories

mod iter;

pub use self::iter::Iter;
use crate::{
    advisory::{self, Advisory},
    error::{Error, ErrorKind},
    lockfile::Lockfile,
    package,
    repository::Repository,
    version::Version,
    vulnerability::Collection,
};
use std::{
    collections::{btree_map as map, BTreeMap as Map},
    ffi::{OsStr, OsString},
};

/// Database of RustSec security advisories, indexed both by ID and crate
#[derive(Debug)]
pub struct Database {
    advisories: Map<advisory::Id, Advisory>,
    crates: Map<package::Name, Vec<advisory::Id>>,
}

impl Database {
    /// Fetch the default advisory database from GitHub
    #[cfg(feature = "chrono")]
    pub fn fetch() -> Result<Self, Error> {
        let repo = Repository::fetch_default_repo()?;
        Self::from_repository(&repo)
    }

    /// Create a new `AdvisoryDatabase` from the given `Repository`
    pub fn from_repository(repo: &Repository) -> Result<Self, Error> {
        let advisory_files = repo.crate_advisories()?;
        let mut advisories = Map::new();
        let mut crates = Map::new();

        for advisory_file in advisory_files {
            let advisory = Advisory::load_file(advisory_file.path())?;

            if !advisory.metadata.id.is_rustsec() {
                fail!(
                    ErrorKind::Parse,
                    "expected a RUSTSEC advisory ID: {}",
                    advisory.metadata.id
                );
            }

            let advisory_path = advisory_file.path().to_owned();
            let expected_filename = OsString::from(format!("{}.toml", advisory.metadata.id));

            // Ensure advisory has the correct filename
            if advisory_path.file_name().unwrap() != expected_filename {
                fail!(
                    ErrorKind::Repo,
                    "expected {} to be named {:?}",
                    advisory_file.path().display(),
                    expected_filename
                );
            }

            // Ensure advisory is in the correct directory
            let advisory_parent_dir = advisory_path.parent().unwrap().file_name().unwrap();

            if advisory_parent_dir != OsStr::new(advisory.metadata.package.as_str()) {
                fail!(
                    ErrorKind::Repo,
                    "expected {} to be in {} directory (instead of \"{:?}\")",
                    advisory.metadata.id,
                    advisory.metadata.package,
                    advisory_parent_dir
                );
            }

            // Ensure placeholder advisories load and parse correctly, but
            // don't actually insert them into the advisory database
            if advisory.metadata.id.is_placeholder() {
                continue;
            }

            let crate_advisories = match crates.entry(advisory.metadata.package.clone()) {
                map::Entry::Vacant(entry) => entry.insert(vec![]),
                map::Entry::Occupied(entry) => entry.into_mut(),
            };

            crate_advisories.push(advisory.metadata.id.clone());
            advisories.insert(advisory.metadata.id.clone(), advisory);
        }

        Ok(Self { advisories, crates })
    }

    /// Look up an advisory by an advisory ID (e.g. "RUSTSEC-YYYY-XXXX")
    pub fn find(&self, id: &advisory::Id) -> Option<&Advisory> {
        self.advisories.get(id)
    }

    /// Look up advisories relevant to a particular crate
    pub fn find_by_crate<N: AsRef<package::Name>>(&self, crate_name: N) -> Vec<&Advisory> {
        if let Some(ids) = self.crates.get(crate_name.as_ref()) {
            ids.iter().map(|id| self.find(&id).unwrap()).collect()
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
    pub fn vulnerabilities(&self, lockfile: &Lockfile) -> Collection {
        Collection::find(self, lockfile)
    }

    /// Iterate over all of the advisories in the database
    pub fn advisories(&self) -> Iter<'_> {
        Iter(self.advisories.iter())
    }
}
