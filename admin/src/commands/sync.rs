//! `rustsec-admin sync` subcommand

use crate::{prelude::*, synchronizer::Synchronizer};
use abscissa_core::{Command, Runnable};
use clap::Parser;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

/// `rustsec-admin sync` subcommand
#[derive(Command, Debug, Default, Parser)]
pub struct SyncCmd {
    /// Path to the advisory database
    #[clap(
        min_values = 1,
        help = "filesystem path to the RustSec advisory DB git repo"
    )]
    path: Vec<PathBuf>,

    /// Path to the OSV export
    //
    // Downloaded with:
    //
    // gsutil cp gs://osv-vulnerabilities/crates.io/all.zip .
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

        let mut synchronizer = Synchronizer::new(repo_path, &self.osv).unwrap_or_else(|e| {
            status_err!(
                "error loading advisory DB repo from {}: {}",
                repo_path.display(),
                e
            );

            exit(1);
        });

        let advisories = synchronizer.advisory_db().iter();

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

        let (updated, mut new) = synchronizer.sync().unwrap_or_else(|e| {
            status_err!(
                "error synchronizing advisory DB {}: {}",
                repo_path.display(),
                e
            );

            exit(1);
        });

        if new.is_empty() {
            status_ok!("Success", "no new advisories to import");
        } else {
            status_ok!("Success", "{} aliases are missing in RustSec", new.len());
            // Only a message from now
            // TODO: automate new advisory draft
            new.sort_by(|a, b| a.published().partial_cmp(b.published()).unwrap());
            for a in new {
                println!(
                    "{:.10}: https://github.com/advisories/{} for {:?}",
                    a.published(),
                    a.id(),
                    a.crates()
                );
            }
        }

        if updated == 0 {
            status_ok!("Success", "all advisories are up to date");
        } else {
            status_ok!("Success", "{} advisories have been updated", updated);
        }
    }
}
