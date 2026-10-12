//! Backend for the `list-affected-versions` subcommand.

use std::path::PathBuf;

use rustsec::{Advisory, Database};

use crate::{crates_index, error::Error, lock::acquire_cargo_package_lock, prelude::*};

/// Lists all versions for a crate and prints info on which ones are affected
pub struct AffectedVersionLister {
    /// Loaded crates.io index
    crates_index: rustsec::registry::RemoteSparseIndex,

    /// Loaded Advisory DB
    advisory_db: Database,
}

impl AffectedVersionLister {
    /// Load the database at the given path
    pub fn new(repo_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let advisory_db = Database::open(&repo_path)?;

        Ok(Self {
            crates_index: crates_index()?,
            advisory_db,
        })
    }

    /// Borrow the loaded advisory database
    pub fn advisory_db(&self) -> &Database {
        &self.advisory_db
    }

    /// List affected and unaffected crate versions for a given advisory
    fn process_one_advisory(&self, advisory: &Advisory, crate_info: tame_index::IndexKrate) {
        status_ok!(
            "Loaded",
            "{} for '{}'",
            advisory.id(),
            advisory.metadata.package
        );

        for version in crate_info.versions {
            let parsed_version = rustsec::Version::parse(&version.version).unwrap();
            if advisory.versions.is_vulnerable(&parsed_version) {
                println!("{} vulnerable", version.version)
            } else {
                println!("{} OK", version.version)
            }
        }
    }

    /// List affected and unaffected crate versions for all advisories
    pub fn process_all_advisories(&self) -> Result<(), Error> {
        let crates = self
            .advisory_db()
            .iter()
            .filter_map(|adv| {
                (adv.metadata.collection.unwrap() == rustsec::Collection::Crates)
                    .then(|| adv.metadata.package.clone())
            })
            .collect();

        let mut metadata = {
            let lock = acquire_cargo_package_lock().unwrap();
            self.crates_index.multi_fetch(crates, None, &lock)?
        };

        for advisory in self.advisory_db.iter() {
            // We currently only support crate versions, not advisories against Rust versions
            if advisory.metadata.collection.unwrap() != rustsec::Collection::Crates {
                continue;
            }

            let name = &advisory.metadata.package;

            let Some(pos) = metadata.iter().position(|(pn, _)| pn == name) else {
                panic!("failed to find result for {name}");
            };

            let ik = match metadata.swap_remove(pos).1 {
                Ok(Some(md)) => md,
                Ok(None) => {
                    panic!("{name} did not exist in the crates.io index");
                }
                Err(err) => {
                    panic!("failed to fetch {name}: {err}");
                }
            };

            self.process_one_advisory(advisory, ik);
        }
        Ok(())
    }
}
