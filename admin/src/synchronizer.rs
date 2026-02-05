//! RustSec Advisory DB Synchronizer
//!
//! Update the RustSec advisories from external sources.
//! We use the OSV format as input, as it is the interoperable standard.
//!
//! ## GitHub Advisory Database
//!
//! Our unique source of external information is the [GitHub Advisory Database](https://github.com/advisories).
//! Their Rust vulnerabilities have various possible origins:
//!
//! * Reported directly to GitHub using their build-in security advisories feature
//! * imported from a CVE, using metadata from [NVD](https://nvd.nist.gov/vuln)
//! * imported from RustSec.
//!   When importing a RustSec inventory, they assign it a GHSA and CVE IDs.
//!
//! The data from this database allows us to:
//!
//! * Find advisories missing in RustSec
//!   * We want to manually review those before importing them, to ensure
//!     the content match our standards and processes.
//! * Add GHSA and CVE aliases to our vulnerabilities.
//!   CVE are specially important
//!   as they are the most use ID for vulnerabilities.
//! * Add missing metadata to our advisories
//!
//! GitHub exposes a GraphQL API, but we chose to use their OSV export as a source.
//!
//! ## osv.dev
//!
//! osv.dev imports from both GitHub Security Advisories and RustSec,
//! and exposes its advisories through both an HTTP API and ZIP files.
//!
//!
//! Workflow:
//!
//! ```text
//!          ┌───────────────────────────────────┐
//!          │                                   │
//!     ┌────┴────┐         ┌─────────┐        ┌─▼────┐
//!     │ RustSec │─────────▶ OSV.dev ◀────────│ GHSA │
//!     └────▲────┘         └────┬────┘        └──────┘
//!          │                   │
//!          └───────────────────┘
//! ```
//!
//! We use the ZIP file export as a source as we need all advisories at once.
//!
//! The file containing crates.io vulnerabilities is available with:
//!
//! ```shell
//! gsutil cp gs://osv-vulnerabilities/crates.io/all.zip .
//! # or
//! curl -o advisories.zip https://osv-vulnerabilities.storage.googleapis.com/crates.io/all.zip
//! ```
//!
//! ## Sync process
//!
//! ### Get aliases for advisories imported from RustSec
//!
//! We can detect advisories imported from RustSec quite reliabilly by looking for a reference to the
//! advisory file in the `advisory-db` repository.
//! In this case, we can also check if there is only one RustSec advisory to make sure
//! it is really an alias.
//!
//! Then we can add the GHSA id and the CVE id as aliases in the RustSec advisory.
//!
//! ## List missing advisories
//!
//! When an advisory contains no reference to an existing RustSec advisory, it is likely
//! missing.

use std::fs::read_to_string;
use std::iter::FromIterator;
use std::{
    fs, iter,
    path::{Path, PathBuf},
};

use rustsec::advisory::{Id, IdKind, Parts};
use rustsec::osv::OsvAdvisory;
use rustsec::{Advisory, Collection};
use tame_index::{KrateName, index::RemoteSparseIndex};
use toml_edit::{DocumentMut, value};

use crate::{
    crates_index,
    error::{Error, ErrorKind},
    lock::acquire_cargo_package_lock,
    prelude::*,
};

/// Advisory synchronizer
#[allow(dead_code)]
pub struct Synchronizer {
    /// Path to the advisory database
    repo_path: PathBuf,

    /// Loaded crates.io index
    crates_index: RemoteSparseIndex,

    /// Loaded Advisory DB
    advisory_db: rustsec::Database,

    /// OSV advisories to synchronize from
    osv: Vec<OsvAdvisory>,

    /// Number of updated advisories
    updated_advisories: usize,

    /// Missing advisories
    missing_advisories: Vec<OsvAdvisory>,
}

impl Synchronizer {
    /// Create a new synchronizer for the database at the given path
    pub fn new(repo_path: impl Into<PathBuf>, osv_path: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_path = repo_path.into();
        let advisory_db = rustsec::Database::open(&repo_path)?;

        let osv = Self::load_osv_export(&osv_path.into())?;
        status_info!(
            "Info",
            "Loaded {} advisories from {}",
            osv.len(),
            repo_path.display()
        );

        Ok(Self {
            repo_path,
            crates_index: crates_index()?,
            advisory_db,
            osv,
            updated_advisories: 0,
            missing_advisories: vec![],
        })
    }

    /// Borrow the loaded advisory database
    pub fn advisory_db(&self) -> &rustsec::Database {
        &self.advisory_db
    }

    /// Synchronize data
    pub fn sync(&mut self) -> Result<(usize, Vec<OsvAdvisory>), Error> {
        // A single OSV advisory could describe a vulnerability affecting several crates
        // (even if GitHub does not produce such advisories currently).
        // Additionally, a single RustSec advisory can cover several OSV advisories
        // depending on the way it was reported.
        // Therefore, we make as few assumptions as possible here.
        for osv in self.osv.clone() {
            if osv.withdrawn() {
                // Ignore withdrawn advisories from the start
                continue;
            }

            // The list of RustSec ids referenced by this OSV advisory,
            // generally one for a GHSA created from RustSec.
            // When imported, they can be considered actual aliases.
            let rustsec_ids_in_osv = osv.rustsec_refs_imported();
            // The list of crates affected by the advisory, normally one
            // for a GHSA created from RustSec.
            let affected_crates = osv.crates();

            // The list of RustSec advisories already having this advisory id as alias
            let rustsec_ids_alias: Vec<Id> = self
                .advisory_db
                .iter()
                .filter_map(|a| {
                    if a.metadata.aliases.contains(osv.id()) {
                        Some(a.id().clone())
                    } else {
                        None
                    }
                })
                .collect();

            // Build the full list of rs aliases
            let mut rs_aliases = rustsec_ids_in_osv.clone();
            rs_aliases.extend(rustsec_ids_alias.clone());
            rs_aliases.sort();
            rs_aliases.dedup();

            // This advisory does not link to RustSec (i.e., was not imported)
            // and is not aliased from RustSec. Let's consider importing it.
            if rs_aliases.is_empty() {
                for c in affected_crates {
                    let crate_name: KrateName = match c.as_str().try_into() {
                        Ok(k) => k,
                        Err(_e) => {
                            status_info!(
                                "Info",
                                "Crate name {} in {} advisory is invalid, skipping",
                                c,
                                osv.id(),
                            );
                            continue;
                        }
                    };

                    if let Ok(Some(_)) = self.crates_index.krate(
                        crate_name,
                        true,
                        &acquire_cargo_package_lock().unwrap(),
                    ) {
                        self.missing_advisories.push(osv.clone());
                    } else {
                        status_info!(
                            "Info",
                            "Unknown crate {} in {} advisory, skipping",
                            c,
                            osv.id()
                        );
                        continue;
                    }
                }
            } else {
                // Update advisories from known links
                for rs_id in rs_aliases {
                    // ensure all these advisories have up-to-date aliases
                    // missing alias to GHSA
                    let rs_advisory = self
                        .advisory_db
                        .get(&rs_id)
                        .expect("Referenced advisory not in rustsec")
                        .clone();

                    // ensure the crate name matches
                    if !affected_crates
                        .iter()
                        .any(|c| c == rs_advisory.metadata.package.as_str())
                    {
                        status_info!(
                            "Info",
                            "Crate names {:?} in {} advisory not matching existing advisory {}, skipping",
                            affected_crates,
                            osv.id(),
                            rs_advisory.id()
                        );
                        continue;
                    }

                    self.update_advisory_from_alias(&rs_advisory, &osv)?;
                }
            }
        }
        Ok((self.updated_advisories, self.missing_advisories.clone()))
    }

    /// Add missing data to advisory from an external source
    ///
    /// For now, only add missing aliases.
    fn update_advisory_from_alias(
        &mut self,
        advisory: &Advisory,
        external: &OsvAdvisory,
    ) -> Result<(), Error> {
        let mut missing_aliases = vec![];
        let missing_related = vec![];
        for external_id in external.aliases().iter().chain(iter::once(external.id())) {
            // Heuristic based on advisory kind
            match external_id.kind() {
                IdKind::Cve | IdKind::Ghsa => {
                    if external_id != advisory.id()
                        && !advisory.metadata.aliases.contains(external_id)
                    {
                        missing_aliases.push(external_id.clone());
                        status_info!(
                            "Info",
                            "Adding missing alias {} for {}",
                            external_id,
                            advisory.id()
                        );
                    }
                }
                _ => continue,
            }
        }
        if !missing_aliases.is_empty() || !missing_related.is_empty() {
            self.update_aliases(
                &self
                    .repo_path
                    .join(Collection::Crates.to_string())
                    .join(advisory.metadata.package.as_str())
                    .join(format!("{}.md", advisory.id())),
                &missing_aliases,
                &missing_related,
            )?;
        }
        Ok(())
    }

    /// Edit advisory file to extend aliases field
    fn update_aliases(
        &mut self,
        advisory_path: &Path,
        missing_aliases: &[Id],
        missing_related: &[Id],
    ) -> Result<(), Error> {
        let content = read_to_string(advisory_path)?;
        // First extract toml and markdown content
        // We can't parse as Advisory as we want to preserve formatting
        let parts = Parts::parse(&content)?;
        // Parse toml
        let mut metadata = parts
            .front_matter
            .parse::<DocumentMut>()
            .expect("invalid TOML front matter");

        // Aliases
        let mut aliases: Vec<String> = metadata["advisory"]
            .get("aliases")
            .map(|i| {
                i.as_array()
                    .unwrap()
                    .into_iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect()
            })
            .unwrap_or_else(Vec::new);
        aliases.extend(missing_aliases.iter().map(|a| a.to_string()));
        aliases.sort();
        aliases.dedup();
        if !aliases.is_empty() {
            metadata["advisory"]["aliases"] = value(toml_edit::Array::from_iter(aliases.iter()));
        }

        // Related
        // FIXME: dedup implementation
        let mut related: Vec<String> = metadata["advisory"]
            .get("related")
            .map(|i| {
                i.as_array()
                    .unwrap()
                    .into_iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect()
            })
            .unwrap_or_else(Vec::new);
        related.extend(missing_related.iter().map(|a| a.to_string()));
        related.sort();
        related.dedup();
        if !related.is_empty() {
            metadata["advisory"]["related"] = value(toml_edit::Array::from_iter(related.iter()));
        }

        let updated = format!("```toml\n{}```\n\n{}", metadata, parts.markdown);
        fs::write(advisory_path, updated)?;
        status_info!("Info", "Written {}", advisory_path.display());
        self.updated_advisories += 1;
        Ok(())
    }

    /// Load an OSV advisory from a JSON file
    fn load_osv_file(path: impl AsRef<Path>) -> Result<OsvAdvisory, Error> {
        let path = path.as_ref();

        let advisory_data = read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?;

        let advisory: OsvAdvisory = serde_json::from_str(&advisory_data).map_err(|e| {
            format_err!(ErrorKind::Parse, "error parsing {}: {}", path.display(), e)
        })?;

        Ok(advisory)
    }

    /// Load data from an OSV export
    fn load_osv_export(path: &Path) -> Result<Vec<OsvAdvisory>, Error> {
        let mut result = vec![];
        for advisory_entry in fs::read_dir(path).unwrap() {
            let advisory_path = advisory_entry.unwrap().path();
            if advisory_path.extension() != Some("json".as_ref()) {
                // Skip non-JSON files
                continue;
            }
            if advisory_path.to_string_lossy().contains("RUSTSEC-") {
                // Don't parse advisories already coming from RustSec
                continue;
            }
            let advisory = Self::load_osv_file(advisory_path)?;
            result.push(advisory)
        }
        Ok(result)
    }
}
