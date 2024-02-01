//! `rustsec-admin list-affected-versions` subcommand
//!
//! Can be used to verify that the version specification in the advisory
//! had the desired effect and matches only the versions you want it to.

#![allow(clippy::never_loop)]
#![allow(unused_variables)] //TODO

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use abscissa_core::{Command, Runnable};
use clap::Parser;

use crate::list_versions::AffectedVersionLister;
use crate::prelude::*;

/// `rustsec-admin list-affected-versions` subcommand
#[derive(Command, Debug, Default, Parser)]
pub struct ListAffectedVersionsCmd {
    /// Path to the advisory database
    #[arg(
        num_args = 1..,
        help = "filesystem path to the RustSec advisory DB git repo"
    )]
    path: Vec<PathBuf>,
}

impl Runnable for ListAffectedVersionsCmd {
    fn run(&self) {
        let repo_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => unreachable!(),
        };

        let lister = AffectedVersionLister::new(repo_path).unwrap_or_else(|e| {
            status_err!(
                "error loading advisory DB repo from {}: {}",
                repo_path.display(),
                e
            );
            exit(1);
        });

        // Ensure we're parsing some advisories
        let advisories = lister.advisory_db().iter();
        if advisories.len() == 0 {
            status_err!("no advisories found in {}", repo_path.display());
            exit(1);
        }

        lister.process_all_advisories().unwrap_or_else(|e| {
            status_err!(
                "error listing affected versions for DB {}: {}",
                repo_path.display(),
                e
            );
            exit(1);
        });
    }
}
