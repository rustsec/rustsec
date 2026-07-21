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

use std::{
    fs::{self, read_to_string},
    iter,
    iter::FromIterator,
    path::{Path, PathBuf},
    process::exit,
};

use abscissa_core::{Command, Runnable};
use clap::Parser;
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

/// `rustsec-admin sync` subcommand
#[derive(Command, Debug, Default, Parser)]
pub(crate) struct SyncCmd {
    /// Path to the advisory database
    #[arg(
        num_args = 1..,
        help = "filesystem path to the RustSec advisory DB git repo"
    )]
    path: Vec<PathBuf>,

    /// Path to the OSV export
    //
    // Downloaded with:
    //
    // gsutil cp gs://osv-vulnerabilities/crates.io/all.zip .
    // or
    // wget https://osv-vulnerabilities.storage.googleapis.com/crates.io/all.zip
    #[clap(
        long = "osv",
        help = "filesystem path to the OSV crates.io data export"
    )]
    osv: PathBuf,
}

impl Runnable for SyncCmd {
    fn run(&self) {
        let repo_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => unreachable!(),
        };

        let Ok(advisory_db) = rustsec::Database::open(repo_path) else {
            status_err!(
                "error loading advisory DB repo from {}",
                repo_path.display()
            );
            exit(1);
        };

        let osv = match load_osv_export(&self.osv) {
            Ok(osv) => osv,
            Err(e) => {
                status_err!(
                    "error loading OSV export from {}: {}",
                    self.osv.display(),
                    e
                );
                exit(1);
            }
        };
        status_info!(
            "Info",
            "Loaded {} advisories from {}",
            osv.len(),
            repo_path.display()
        );

        let Ok(crates_index) = crates_index() else {
            status_err!("error loading crates.io index");
            exit(1);
        };

        let advisories = advisory_db.iter();

        // Ensure we're parsing some advisories
        if advisories.len() == 0 {
            status_err!("no advisories found!");
            exit(1);
        }

        status_ok!(
            "Loaded",
            "{} security advisories (from {})",
            advisories.len(),
            repo_path.display()
        );

        let mut synchronized =
            sync(osv, &advisory_db, &repo_path, &crates_index).unwrap_or_else(|e| {
                status_err!(
                    "error synchronizing advisory DB {}: {}",
                    repo_path.display(),
                    e
                );

                exit(1);
            });

        if synchronized.missing_advisories.is_empty() {
            status_ok!("Success", "no new advisories to import");
        } else {
            status_ok!(
                "Success",
                "{} aliases are missing in RustSec",
                synchronized.missing_advisories.len()
            );
            // Only a message from now
            // TODO: automate new advisory draft
            synchronized
                .missing_advisories
                .sort_by(|a, b| a.published().partial_cmp(b.published()).unwrap());
            for a in synchronized.missing_advisories {
                println!(
                    "{:.10}: https://github.com/advisories/{} for {:?}",
                    a.published(),
                    a.id(),
                    a.crates()
                );
            }
        }

        if synchronized.updated_advisories == 0 {
            status_ok!("Success", "all advisories are up to date");
        } else {
            status_ok!(
                "Success",
                "{} advisories have been updated",
                synchronized.updated_advisories
            );
        }
    }
}

/// Synchronize data
fn sync(
    osv: Vec<OsvAdvisory>,
    advisory_db: &rustsec::Database,
    repo_path: &Path,
    crates_index: &RemoteSparseIndex,
) -> Result<Synchronized, Error> {
    // A single OSV advisory could describe a vulnerability affecting several crates
    // (even if GitHub does not produce such advisories currently).
    // Additionally, a single RustSec advisory can cover several OSV advisories
    // depending on the way it was reported.
    // Therefore, we make as few assumptions as possible here.
    let mut out = Synchronized::default();
    for osv in osv {
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
        let rustsec_ids_alias: Vec<Id> = advisory_db
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
                let crate_name = match KrateName::try_from(c.as_str()) {
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

                if let Ok(Some(_)) =
                    crates_index.krate(crate_name, true, &acquire_cargo_package_lock().unwrap())
                {
                    out.missing_advisories.push(osv.clone());
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
                let rs_advisory = advisory_db
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

                update_advisory_from_alias(&rs_advisory, osv, &mut out, repo_path)?;
            }
        }
    }
    Ok(out)
}

/// Add missing data to advisory from an external source
///
/// For now, only add missing aliases.
fn update_advisory_from_alias(
    advisory: &Advisory,
    external: &OsvAdvisory,
    out: &mut Synchronized,
    repo_path: &Path,
) -> Result<(), Error> {
    let mut missing_aliases = vec![];
    let missing_related = vec![];
    for external_id in external.aliases().iter().chain(iter::once(external.id())) {
        // Heuristic based on advisory kind
        match external_id.kind() {
            IdKind::Cve | IdKind::Ghsa => {
                if external_id != advisory.id() && !advisory.metadata.aliases.contains(external_id)
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

    if missing_aliases.is_empty() && missing_related.is_empty() {
        return Ok(());
    }

    let advisory_path = repo_path
        .join(Collection::Crates.to_string())
        .join(advisory.metadata.package.as_str())
        .join(format!("{}.md", advisory.id()));

    let content = read_to_string(&advisory_path)?;
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
    related.extend(missing_related);
    related.sort();
    related.dedup();
    if !related.is_empty() {
        metadata["advisory"]["related"] = value(toml_edit::Array::from_iter(related.iter()));
    }

    let updated = format!("```toml\n{}```\n\n{}", metadata, parts.markdown);
    fs::write(&advisory_path, updated)?;
    status_info!("Info", "Written {}", advisory_path.display());
    out.updated_advisories += 1;
    Ok(())
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
        let advisory = load_osv_file(advisory_path)?;
        result.push(advisory)
    }
    Ok(result)
}

/// Load an OSV advisory from a JSON file
fn load_osv_file(path: impl AsRef<Path>) -> Result<OsvAdvisory, Error> {
    let path = path.as_ref();

    let advisory_data = read_to_string(path)
        .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?;

    let advisory: OsvAdvisory = serde_json::from_str(&advisory_data)
        .map_err(|e| format_err!(ErrorKind::Parse, "error parsing {}: {}", path.display(), e))?;

    Ok(advisory)
}

#[derive(Default)]
struct Synchronized {
    pub updated_advisories: usize,
    pub missing_advisories: Vec<OsvAdvisory>,
}
