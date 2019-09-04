//! Entries in the advisory database

use super::iter::Iter;
use crate::{
    advisory::{self, Advisory},
    error::{Error, ErrorKind},
    map, package, Map,
};
use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

/// Entries in the advisory database, indexed by the advisory ID as the
/// primary key.
#[derive(Debug, Default)]
pub struct Entries(Map<advisory::Id, Advisory>);

impl Entries {
    /// Create a new database entries collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Load an advisory from a file and insert it into the database entries.
    ///
    /// Errors if the advisory ID is a duplicate.
    pub fn load_file(&mut self, path: &Path) -> Result<Option<&Advisory>, Error> {
        let mut advisory = Advisory::load_file(path)?;
        let expected_filename = OsString::from(format!("{}.toml", advisory.metadata.id));

        // Ensure advisory has the correct filename
        if path.file_name().unwrap() != expected_filename {
            fail!(
                ErrorKind::Repo,
                "expected {} to be named {:?}",
                path.display(),
                expected_filename
            );
        }

        // Ensure advisory is in a directory named after its package
        let package_dir = path.parent().ok_or_else(|| {
            format_err!(
                ErrorKind::Repo,
                "advisory has no parent dir: {}",
                path.display()
            )
        })?;

        if package_dir.file_name().unwrap() != OsStr::new(advisory.metadata.package.as_str()) {
            fail!(
                ErrorKind::Repo,
                "expected {} to be in {} directory (instead of \"{:?}\")",
                advisory.metadata.id,
                advisory.metadata.package,
                package_dir
            );
        }

        // Get the collection this advisory is part of
        let collection_dir = package_dir
            .parent()
            .ok_or_else(|| {
                format_err!(
                    ErrorKind::Repo,
                    "advisory has no collection: {}",
                    path.display()
                )
            })?
            .file_name()
            .unwrap();

        let collection = if collection_dir == OsStr::new(package::Collection::Crates.as_str()) {
            package::Collection::Crates
        } else if collection_dir == OsStr::new(package::Collection::Rust.as_str()) {
            package::Collection::Rust
        } else {
            fail!(
                ErrorKind::Repo,
                "invalid package collection: {:?}",
                collection_dir
            );
        };

        match advisory.metadata.collection {
            Some(c) => {
                if c != collection {
                    fail!(
                        ErrorKind::Parse,
                        "collection mismatch for {}",
                        &advisory.metadata.id
                    );
                }
            }
            None => advisory.metadata.collection = Some(collection),
        }

        // Ensure placeholder advisories load and parse correctly, but
        // don't actually insert them into the advisory database
        if advisory.metadata.id.is_placeholder() {
            return Ok(None);
        }

        match self.0.entry(advisory.metadata.id.clone()) {
            map::Entry::Vacant(entry) => Ok(Some(entry.insert(advisory))),
            map::Entry::Occupied(entry) => {
                fail!(ErrorKind::Parse, "duplicate advisory ID: {}", entry.key())
            }
        }
    }

    /// Get an advisory from the database by its ID
    pub fn get(&self, id: &advisory::Id) -> Option<&Advisory> {
        self.0.get(id)
    }

    /// Iterate over all of the entries in the database
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.0.iter())
    }
}
