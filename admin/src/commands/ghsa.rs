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
        let repo_path: Option<&Path> = self.repo_path.as_deref();
        let token = std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| {
            status_err!("GITHUB_TOKEN env variable must be set");
            exit(1);
        });
        ghsa::fetch_stuff(&token);
        // let exporter = OsvExporter::new(repo_path).unwrap_or_else(|e| {
        //     status_err!("Failed to fetch the advisory database: {}", e);
        //     exit(1);
        // });
        // exporter.export_all(out_path).unwrap_or_else(|e| {
        //     status_err!("failed not export to '{}': {}", out_path.display(), e);
        //     exit(1);
        // });
    }
}