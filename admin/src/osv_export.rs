//! Backend for the `osv` subcommand.

#![allow(warnings)] //TODO
#![warn(warnings)]

use std::path::{Path, PathBuf};

use rustsec::{Database, osv::OsvAdvisory, repository::git::{Repository, GitModificationTimes}};

use crate::{error::Error, prelude::*};

/// Lists all versions for a crate and prints info on which ones are affected
pub struct OsvExporter {
    /// Loaded Advisory DB
    advisory_db: Database,

    /// Loaded modification times for files in Git
    mod_times: GitModificationTimes,
}

impl OsvExporter {
    /// Load the the database at the given path
    pub fn new(repo_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let repo = Repository::open(&repo_path)?;
        let advisory_db = Database::open(&repo_path)?;
        let mod_times = GitModificationTimes::new(&repo)?;
        Ok(Self {
            advisory_db,
            mod_times
        })
    }

    /// Borrow the loaded advisory database
    pub fn advisory_db(&self) -> &Database {
        &self.advisory_db
    }

    /// Exports all advisories to OSV JSON format at the specified prefix
    pub fn export_all(self, path: &Path) -> Result<(), Error> {
        let mod_times = self.mod_times;
        // This won't work, we need to iterate over files on a lower level
        // for advisory in self.advisory_db.into_iter() {
        //     OsvAdvisory::from_rustsec(advisory, &mod_times, path_to_advisory_file);
        // }
        Ok(())
    }
}