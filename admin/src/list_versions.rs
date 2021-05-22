//! Backend for the `list-affected-versions` subcommand.

#![allow(missing_docs)] //TODO

use std::path::PathBuf;

use crates_index::Index;
use rustsec::{Advisory, Database};

use crate::{error::Error, prelude::*};

pub struct AffectedVersionLister {
    /// Loaded crates.io index
    crates_index: Index,

    /// Loaded Advisory DB
    advisory_db: Database,
}

impl AffectedVersionLister {
    pub fn new(repo_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let crates_index = crates_index::Index::new_cargo_default();
        crates_index.retrieve_or_update()?;
        let advisory_db = Database::open(&repo_path)?;
        Ok(Self {
            crates_index,
            advisory_db,
        })
    }

    /// Borrow the loaded advisory database
    pub fn advisory_db(&self) -> &Database {
        &self.advisory_db
    }

    pub fn process_one_advisory(&self, advisory: &Advisory) {
        status_ok!("Loaded", "{}", advisory.id());
        let crate_name = advisory.metadata.package.as_str();
        let crate_info = self.crates_index.crate_(crate_name).unwrap();
        for version in crate_info.versions() {
            let parsed_version = rustsec::Version::parse(version.version()).unwrap();
            if advisory.versions.is_vulnerable(&parsed_version) {
                println!("{} vulnerable", version.version())
            } else {
                println!("{} OK", version.version())
            }
        }
    }

    pub fn process_all_advisories(&self) -> Result<(), Error> {
        let crates_index = crates_index::Index::new_cargo_default();
        crates_index.retrieve_or_update()?;

        for advisory in self.advisory_db.iter() {
            // We currently only support crate versions, not advisories against Rust versions
            if advisory.metadata.collection.unwrap() != rustsec::Collection::Crates {
                continue;
            }
            self.process_one_advisory(advisory);
        }
        Ok(())
    }
}
