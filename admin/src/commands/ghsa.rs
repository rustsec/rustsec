//! `rustsec-admin ghsa` subcommand
//!
//! Imports advisory data from GHSA to RustSec.
//!
//! Requires the GITHUB_TOKEN environment variable to be set, see
//! https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
//! Do not grant any access scopes to the token! We only need read-only access to public data.

#![allow(unused_variables)] //TODO
#![allow(unused_imports)] //TODO

use std::{
    path::{Path, PathBuf},
    process::exit,
};

use abscissa_core::{status_err, Command, Options, Runnable};

use crate::ghsa;

#[derive(Command, Debug, Default, Options)]
pub struct GhsaCmd {
    /// Path to the advisory database
    #[options(
        long = "db",
        help = "filesystem path to the RustSec advisory DB git repo"
    )]
    repo_path: Option<PathBuf>,
}

impl Runnable for GhsaCmd {
    fn run(&self) {
        let repo_path = self.repo_path.clone();
        let token = std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| {
            status_err!("GITHUB_TOKEN env variable must be set");
            exit(1);
        });
        let importer = ghsa::GhsaImporter::new(repo_path).unwrap();
        importer.do_stuff(&token);
    }
}
