//! RustSec Advisory DB Linter

use crate::{
    error::{Error, ErrorKind},
    prelude::*,
};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

/// List of "collections" within the Advisory DB
// TODO(tarcieri): provide some other means of iterating over the collections?
pub const COLLECTIONS: &[rustsec::Collection] =
    &[rustsec::Collection::Crates, rustsec::Collection::Rust];

/// Advisory linter
pub struct Linter {
    /// Path to the advisory database
    repo_path: PathBuf,

    /// HTTP client for crates.io requests, persisted for connection pooling
    http_client: ureq::Agent,

    /// Loaded Advisory DB
    advisory_db: rustsec::Database,

    /// Total number of invalid advisories encountered
    invalid_advisories: usize,
}

impl Linter {
    /// Create a new linter for the database at the given path
    pub fn new(repo_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let http_client = ureq::AgentBuilder::new()
            .user_agent("RustSec advisory database linter")
            .timeout(Duration::from_secs(20))
            .build();
        let advisory_db = rustsec::Database::open(&repo_path)?;

        Ok(Self {
            repo_path,
            http_client,
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
        if !self.name_exists_on_crates_io(advisory.metadata.package.as_str())? {
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
    fn name_exists_on_crates_io(&self, name: &str) -> Result<bool, Error> {
        // This contains a homebrew crates.io API client.
        // It was created because `crates_io_api` is bloated (async!)
        // and breaks any time crates.io changes any fields at all,
        // even the ones we don't use. And we literally need ONE field.

        #[derive(serde::Deserialize)]
        struct CrateResponse {
            #[serde(alias = "crate")] // "crate" is a reserved keyword and cannot be used as a name
            crate_info: CrateInfo, // there are more fields, but this is the only one we need
        }

        #[derive(serde::Deserialize)]
        struct CrateInfo {
            name: String, // there are more fields, but this is the only one we need
        }

        let url = format!("https://crates.io/api/v1/crates/{}", name);
        let response: CrateResponse = self.http_client.get(&url).call()?.into_json()?;
        // FIXME: I've ported this equality check from legacy code
        // and I have no idea what it does or why it's needed
        Ok(response.crate_info.name == name)
    }
}
