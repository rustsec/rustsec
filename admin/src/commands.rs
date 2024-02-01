//! `rustsec-admin` CLI subcommands

mod assign_id;
mod lint;
mod list_affected_versions;
mod osv;
mod version;
mod web;

use self::{
    assign_id::AssignIdCmd, lint::LintCmd, list_affected_versions::ListAffectedVersionsCmd,
    osv::OsvCmd, version::VersionCmd, web::WebCmd,
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
