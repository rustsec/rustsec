//! RustSec Advisory DB Linter

use crate::{
    error::{Error, ErrorKind},
    lock::acquire_cargo_package_lock,
    prelude::*,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tame_index::index::RemoteGitIndex;

/// List of "collections" within the Advisory DB
// TODO(tarcieri): provide some other means of iterating over the collections?
pub const COLLECTIONS: &[rustsec::Collection] =
    &[rustsec::Collection::Crates, rustsec::Collection::Rust];

/// Advisory linter
pub struct Linter {
    /// Path to the advisory database
    repo_path: PathBuf,

    /// Loaded crates.io index
    crates_index: RemoteGitIndex,

    /// Skip namecheck list
    skip_namecheck: Vec<String>,
}

impl Linter {
    /// Create a new linter for the database at the given path
    pub fn new(
        repo_path: impl Into<PathBuf>,
        skip_namecheck: Option<String>,
    ) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let cargo_package_lock = acquire_cargo_package_lock()?;
        let mut crates_index = RemoteGitIndex::new(
            tame_index::GitIndex::new(tame_index::IndexLocation::new(
                tame_index::IndexUrl::CratesIoGit,
            ))?,
            &cargo_package_lock,
        )?;
        crates_index.fetch(&cargo_package_lock)?;

        Ok(Self {
            repo_path,
            crates_index,
            skip_namecheck: match skip_namecheck {
                Some(s) => s.split(',').map(|s| s.to_owned()).collect(),
                None => Vec::new(),
            },
        })
    }

    /// Lint the loaded database
    pub fn lint(mut self) -> Result<(usize, usize), Error> {
        let (mut valid, mut invalid) = (0, 0);
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
                    match self.is_valid(*collection, &advisory_path)? {
                        true => valid += 1,
                        false => invalid += 1,
                    }
                }
            }
        }

        Ok((valid, invalid))
    }

    /// Lint an advisory at the specified path
    // TODO(tarcieri): separate out presentation (`status_*`) from linting code?
    fn is_valid(
        &mut self,
        collection: rustsec::Collection,
        advisory_path: &Path,
    ) -> Result<bool, Error> {
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

        let lint_result = rustsec::advisory::Linter::lint_file(advisory_path)?;

        if lint_result.errors().is_empty() {
            status_ok!("Linted", "ok: {}", advisory_path.display());
        } else {
            status_err!(
                "{} contained the following lint errors:",
                advisory_path.display()
            );

            for error in lint_result.errors() {
                println!("  - {error}");
            }
        }

        Ok(lint_result.errors().is_empty())
    }

    /// Perform lints that connect to https://crates.io
    fn crates_io_lints(&mut self, advisory: &rustsec::Advisory) -> Result<(), Error> {
        let crate_name = advisory.metadata.package.as_str();
        if self.skip_namecheck.iter().any(|name| name == crate_name) {
            return Ok(());
        }

        // Check if a crate with this name exists on crates.io

        let result = self.crates_index.krate(
            crate_name.try_into().unwrap(),
            true,
            &acquire_cargo_package_lock().unwrap(),
        );

        match result {
            // This check verifies name normalization.
            // A request for "serde-json" might return "serde_json",
            // and we want to catch use a non-canonical name and report it as an error.
            Ok(Some(krate)) if krate.name() == crate_name => Ok(()),
            _ => {
                fail!(
                    ErrorKind::CratesIo,
                    "crates.io package name does not match package name in advisory for {} in {}",
                    advisory.metadata.package.as_str(),
                    advisory.metadata.id
                );
            }
        }
    }
}
