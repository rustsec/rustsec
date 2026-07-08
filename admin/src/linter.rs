//! RustSec Advisory DB Linter

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use rustsec::{Advisory, Collection, Database};
use tame_index::index::RemoteSparseIndex;

use crate::{
    crates_index,
    error::{Error, ErrorKind},
    lock::acquire_cargo_package_lock,
    prelude::*,
};

/// List of "collections" within the Advisory DB
// TODO(tarcieri): provide some other means of iterating over the collections?
pub const COLLECTIONS: &[Collection] = &[Collection::Crates, Collection::Rust];

/// Advisory linter
pub struct Linter {
    /// Path to the advisory database
    repo_path: PathBuf,

    /// Loaded crates.io index
    crates_index: RemoteSparseIndex,

    /// Loaded Advisory DB
    advisory_db: Database,

    /// Total number of invalid advisories encountered
    invalid_advisories: usize,
}

impl Linter {
    /// Create a new linter for the database at the given path
    pub fn new(repo_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let advisory_db = Database::open(&repo_path)?;

        Ok(Self {
            repo_path,
            crates_index: crates_index()?,
            advisory_db,
            invalid_advisories: 0,
        })
    }

    /// Borrow the loaded advisory database
    pub fn advisory_db(&self) -> &Database {
        &self.advisory_db
    }

    /// Lint the loaded database
    pub fn lint(mut self, verbose: bool) -> Result<usize, Error> {
        for collection in COLLECTIONS {
            let crate_entries = read_dir_sorted(self.repo_path.join(collection.as_str()))?;
            let mut advisories = Vec::with_capacity(crate_entries.len());
            for crate_entry in &crate_entries {
                let crate_dir = crate_entry.path();

                if !crate_dir.is_dir() {
                    fail!(
                        ErrorKind::RustSec,
                        "unexpected file in `{}`: {}",
                        collection,
                        crate_dir.display()
                    );
                }

                for advisory_entry in read_dir_sorted(crate_dir)? {
                    let advisory_path = advisory_entry.path();
                    advisories.push(self.lint_advisory(*collection, &advisory_path, verbose)?);
                }
            }

            if collection == &Collection::Crates {
                self.crates_io_lints(&advisories)?;
            }
        }

        Ok(self.invalid_advisories)
    }

    /// Lint an advisory at the specified path
    // TODO(tarcieri): separate out presentation (`status_*`) from linting code?
    fn lint_advisory(
        &mut self,
        collection: Collection,
        advisory_path: &Path,
        verbose: bool,
    ) -> Result<Advisory, Error> {
        if !advisory_path.is_file() {
            fail!(
                ErrorKind::RustSec,
                "unexpected entry in `{}`: {}",
                collection,
                advisory_path.display()
            );
        }

        let advisory = Advisory::load_file(advisory_path)?;
        let lint_result = rustsec::advisory::Linter::lint_file(advisory_path)?;
        if lint_result.errors().is_empty() {
            if verbose {
                status_ok!("Linted", "ok: {}", advisory_path.display());
            }
            return Ok(advisory);
        }

        self.invalid_advisories += 1;
        status_err!(
            "{} contained the following lint errors:",
            advisory_path.display()
        );
        for error in lint_result.errors() {
            println!("  - {error}");
        }

        Ok(advisory)
    }

    /// Perform lints that connect to https://crates.io
    fn crates_io_lints(&mut self, advisories: &[Advisory]) -> Result<(), Error> {
        let names = BTreeSet::from_iter(advisories.iter().filter_map(|advisory| {
            match advisory.metadata.expect_deleted {
                false => Some(advisory.metadata.package.as_str().to_owned()),
                true => None,
            }
        }));

        let lock = match acquire_cargo_package_lock() {
            Ok(lock) => lock,
            Err(error) => {
                status_err!("Failed to acquire cargo package lock: {}", error);
                status_err!("Skipping crates.io lints");
                return Ok(());
            }
        };

        // Check if crates with these names exist on crates.io

        let metadata = self.crates_index.krates(names, true, &lock);
        for advisory in advisories {
            if advisory.metadata.expect_deleted {
                continue;
            }

            match metadata.get(advisory.metadata.package.as_str()) {
                Some(Ok(Some(crate_))) => {
                    // This check verifies name normalization.
                    // A request for "serde-json" might return "serde_json",
                    // and we want to catch use a non-canonical name and report it as an error.
                    if crate_.name() != advisory.metadata.package.as_str() {
                        self.invalid_advisories += 1;

                        fail!(
                            ErrorKind::CratesIo,
                            "crates.io package name does not match package name in advisory for {} in {}",
                            advisory.metadata.package.as_str(),
                            advisory.metadata.id
                        );
                    }
                }
                Some(Ok(None)) | None => {
                    self.invalid_advisories += 1;

                    fail!(
                        ErrorKind::CratesIo,
                        "crates.io package name does not exist for {} in {}",
                        advisory.metadata.package.as_str(),
                        advisory.metadata.id
                    );
                }
                Some(Err(error)) => {
                    self.invalid_advisories += 1;

                    fail!(
                        ErrorKind::CratesIo,
                        "failed to fetch crates.io metadata for {} in {}: {error}",
                        advisory.metadata.package.as_str(),
                        advisory.metadata.id
                    );
                }
            }
        }

        Ok(())
    }
}

fn read_dir_sorted<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<fs::DirEntry>> {
    let read_dir = fs::read_dir(path)?;
    let mut crate_entries = read_dir.into_iter().collect::<Result<Vec<_>, _>>()?;
    crate_entries.sort_by_key(fs::DirEntry::path);
    Ok(crate_entries)
}
