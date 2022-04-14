//! RustSec Advisory DB Linter

use crate::{
    error::{Error, ErrorKind},
    prelude::*,
};
use crates_index::Index;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// List of "collections" within the Advisory DB
// TODO(tarcieri): provide some other means of iterating over the collections?
pub const COLLECTIONS: &[rustsec::Collection] =
    &[rustsec::Collection::Crates, rustsec::Collection::Rust];

/// Advisory linter
pub struct Linter {
    /// Path to the advisory database
    repo_path: PathBuf,

    /// Loaded crates.io index
    crates_index: Index,

    /// Loaded Advisory DB
    advisory_db: rustsec::Database,

    /// Total number of invalid advisories encountered
    invalid_advisories: usize,
}

impl Linter {
    /// Create a new linter for the database at the given path
    pub fn new(repo_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let mut crates_index = crates_index::Index::new_cargo_default()?;
        crates_index.update()?;
        let advisory_db = rustsec::Database::open(&repo_path)?;

        Ok(Self {
            repo_path,
            crates_index,
            advisory_db,
            invalid_advisories: 0,
        })
    }

    /// Borrow the loaded advisory database
    pub fn advisory_db(&self) -> &rustsec::Database {
        &self.advisory_db
    }

    /// Lint the loaded database
    pub fn lint(mut self) -> Result<usize, Error> {
        for collection in COLLECTIONS {
            for crate_entry in fs::read_dir(self.repo_path.join(collection.as_str())).unwrap() {
                let crate_dir = crate_entry.unwrap().path();

                if !crate_dir.is_dir() {
                    fail!(
                        ErrorKind::RustSec,
                        "unexpected file in `{}`: {}",
                        collection,
                        crate_dir.display()
                    );
                }

                for advisory_entry in crate_dir.read_dir().unwrap() {
                    let advisory_path = advisory_entry.unwrap().path();
                    self.lint_advisory(*collection, &advisory_path)?;
                }
            }
        }

        Ok(self.invalid_advisories)
    }

    /// Lint an advisory at the specified path
    // TODO(tarcieri): separate out presentation (`status_*`) from linting code?
    fn lint_advisory(
        &mut self,
        collection: rustsec::Collection,
        advisory_path: &Path,
    ) -> Result<(), Error> {
        if !advisory_path.is_file() {
            fail!(
                ErrorKind::RustSec,
                "unexpected entry in `{}`: {}",
                collection,
                advisory_path.display()
            );
        }

        let advisory = rustsec::Advisory::load_file(advisory_path)?;

        if collection == rustsec::Collection::Crates {
            self.crates_io_lints(&advisory)?;
        }

        let lint_result = rustsec::advisory::Linter::lint_file(&advisory_path)?;

        if lint_result.errors().is_empty() {
            status_ok!("Linted", "ok: {}", advisory_path.display());
        } else {
            self.invalid_advisories += 1;

            status_err!(
                "{} contained the following lint errors:",
                advisory_path.display()
            );

            for error in lint_result.errors() {
                println!("  - {}", error);
            }
        }

        Ok(())
    }

    /// Perform lints that connect to https://crates.io
    fn crates_io_lints(&mut self, advisory: &rustsec::Advisory) -> Result<(), Error> {
        if !self.name_exists_on_crates_io(advisory.metadata.package.as_str()) {
            self.invalid_advisories += 1;

            fail!(
                ErrorKind::CratesIo,
                "crates.io package name does not match package name in advisory for {}",
                advisory.metadata.package.as_str()
            );
        }

        Ok(())
    }

    /// Checks if a crate with this name is present on crates.io
    fn name_exists_on_crates_io(&self, name: &str) -> bool {
        if let Some(crate_) = self.crates_index.crate_(name) {
            // This check verifies name normalization.
            // A request for "serde-json" might return "serde_json",
            // and we want to catch use a non-canonical name and report it as an error.
            crate_.name() == name
        } else {
            false
        }
    }
}
