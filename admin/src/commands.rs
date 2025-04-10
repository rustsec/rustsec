//! `rustsec-admin` CLI subcommands

// This is fired for code expanded from a derive macro,
// but there is nothing explaining what the problem with this is
// and what this pattern is bad for other than being less readable when written by humans.
// https://github.com/rust-lang/rust/issues/120363
// https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#non-local-definitions
// TODO: move this into abscissa-derive
#![allow(unknown_lints)] // don't warn/error on older rustc since non_local_definitions is a recent lint
#![allow(non_local_definitions)]

mod assign_id;
mod lint;
mod list_affected_versions;
mod osv;
mod sync;
mod version;
mod web;

use self::{
    assign_id::AssignIdCmd, lint::LintCmd, list_affected_versions::ListAffectedVersionsCmd,
    osv::OsvCmd, sync::SyncCmd, version::VersionCmd, web::WebCmd,
};
use crate::config::AppConfig;
use abscissa_core::{Command, Configurable, Runnable};
use clap::Parser;
use std::path::PathBuf;

/// `rustsec-admin` CLI subcommands
#[derive(Command, Debug, Parser, Runnable)]
pub enum AdminSubCmd {
    /// The `lint` subcommand
    #[command(about = "lint Advisory DB and ensure is well-formed")]
    Lint(LintCmd),

    /// The `sync` subcommand
    #[clap(about = "synchronize information from external sources (osv.dev, NVD, etc.)")]
    Sync(SyncCmd),

    /// The `web` subcommand
    #[command(about = "render advisory Markdown files for the rustsec.org web site")]
    Web(WebCmd),

    /// The `version` subcommand
    #[command(about = "display version information")]
    Version(VersionCmd),

    /// The `assign-id` subcommand
    #[command(about = "assigning RUSTSEC ids to new vulnerabilities")]
    AssignId(AssignIdCmd),

    /// The `osv` subcommand
    #[command(about = "export advisories to OSV format")]
    Osv(OsvCmd),

    /// The `version` subcommand
    #[command(about = "list affected crate versions")]
    ListAffectedVersions(ListAffectedVersionsCmd),
}

/// `rustsec-admin` CLI commands
#[derive(Command, Debug, Parser)]
#[command(author, version, about)]
pub struct AdminCmd {
    #[command(subcommand)]
    cmd: AdminSubCmd,

    /// Increase verbosity setting
    #[arg(short = 'v', long, help = "Increase verbosity")]
    pub verbose: bool,
}

impl Runnable for AdminCmd {
    fn run(&self) {
        self.cmd.run()
    }
}

impl Configurable<AppConfig> for AdminCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
