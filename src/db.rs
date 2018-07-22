//! Database containing `RustSec` security advisories

use semver::Version;
use std::collections::{
    btree_map::{Entry, Iter as BTMapIter},
    BTreeMap,
};
use std::ffi::{OsStr, OsString};
use toml;

use advisory::{Advisory, AdvisoryId, AdvisoryWrapper};
use error::{Error, ErrorKind};
use package::PackageName;
use repository::Repository;

/// Placeholder advisory name: shouldn't be used until an ID is assigned
pub const PLACEHOLDER_ADVISORY_ID: &str = "RUSTSEC-0000-0000";

/// A collection of security advisories, indexed both by ID and crate
#[derive(Debug)]
pub struct AdvisoryDatabase {
    advisories: BTreeMap<AdvisoryId, Advisory>,
    crates: BTreeMap<PackageName, Vec<AdvisoryId>>,
}

impl AdvisoryDatabase {
    /// Fetch the default advisory database from GitHub
    #[cfg(feature = "chrono")]
    pub fn fetch() -> Result<Self, Error> {
        let repo = Repository::fetch_default_repo()?;
        Self::from_repository(&repo)
    }

    /// Create a new `AdvisoryDatabase` from the given `Repository`
    pub fn from_repository(repo: &Repository) -> Result<Self, Error> {
        let mut advisories = BTreeMap::new();
        let mut crates = BTreeMap::new();

        for advisory_file in repo.crate_advisories()? {
            let AdvisoryWrapper { advisory } = toml::from_str(&advisory_file.read_to_string()?)?;

            let advisory_path = advisory_file.path().to_owned();
            let expected_filename = OsString::from(format!("{}.toml", advisory.id));

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

            if advisory_parent_dir != OsStr::new(advisory.package.as_ref()) {
                fail!(
                    ErrorKind::Repo,
                    "expected {} to be in {} directory (instead of \"{:?}\")",
                    advisory.id,
                    advisory.package,
                    advisory_parent_dir
                );
            }

            // Ensure placeholder advisories load and parse correctly, but
            // don't actually insert them into the advisory database
            if advisory.id.as_ref() != PLACEHOLDER_ADVISORY_ID {
                let mut crate_advisories = match crates.entry(advisory.package.clone()) {
                    Entry::Vacant(entry) => entry.insert(vec![]),
                    Entry::Occupied(entry) => entry.into_mut(),
                };

                crate_advisories.push(advisory.id.clone());
                advisories.insert(advisory.id.clone(), advisory.clone());
            }
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
