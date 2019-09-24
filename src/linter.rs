//! RustSec Advisory DB Linter

use crate::prelude::*;
use std::{path::Path, process::exit};

/// Lint all advisories in the database
pub fn lint_advisories(repo_path: &Path) {
    let repo = rustsec::Repository::open(repo_path).unwrap_or_else(|e| {
        status_err!(
            "couldn't open advisory DB repo from {}: {}",
            repo_path.display(),
            e
        );
        exit(1);
    });

    // Ensure Advisories.toml parses
    let db = rustsec::Database::load(&repo).unwrap();
    let advisories = db.iter();

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

    let cratesio_client = crates_io_api::SyncClient::new();
    let mut invalid_advisories = 0;

    for advisory in advisories {
        if !lint_advisory(repo_path, &cratesio_client, advisory) {
            invalid_advisories += 1;
        }
    }

    if invalid_advisories == 0 {
        status_ok!("Success", "all advisories are well-formed");
    } else {
        status_err!("{} advisories contain errors!", invalid_advisories);
        exit(1);
    }
}

/// Lint an individual advisory in the database
fn lint_advisory(
    repo_path: &Path,
    cratesio_client: &crates_io_api::SyncClient,
    advisory: &rustsec::Advisory,
) -> bool {
    if advisory.metadata.collection == Some(rustsec::Collection::Crates) {
        match cratesio_client.get_crate(advisory.metadata.package.as_str()) {
            Ok(response) => {
                if response.crate_data.name != advisory.metadata.package.as_str() {
                    status_err!(
                        "crates.io package name does not match package name in advisory for {}",
                        advisory.metadata.package.as_str()
                    );
                    return false;
                }
            }
            Err(err) => {
                status_err!(
                    "failed to get package `{}` from crates.io: {}",
                    advisory.metadata.package.as_str(),
                    err
                );
                return false;
            }
        }
    }

    let mut advisory_path = repo_path
        .join(advisory.metadata.collection.as_ref().unwrap().to_string())
        .join(advisory.metadata.package.as_str())
        .join(advisory.metadata.id.as_str());

    advisory_path.set_extension("toml");

    let lint = rustsec::advisory::Linter::lint_file(&advisory_path).unwrap();

    if lint.errors().is_empty() {
        status_ok!(
            "Checked",
            "{} passed lint successfully",
            advisory_path.display()
        );
        true
    } else {
        status_err!(
            "{} contained the following lint errors:",
            advisory_path.display()
        );

        for error in lint.errors() {
            println!("  - {}", error);
        }

        false
    }
}
