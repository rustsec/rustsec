//! `rustsec-admin` CLI subcommands

mod assign_id;
mod ghsa;
mod lint;
mod list_affected_versions;
mod osv;
mod version;
mod web;

use self::{
    assign_id::AssignIdCmd, ghsa::GhsaCmd, lint::LintCmd,
    list_affected_versions::ListAffectedVersionsCmd, osv::OsvCmd, version::VersionCmd, web::WebCmd,
};
use crate::config::AppConfig;
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;

/// `rustsec-admin` CLI subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum AdminCmd {
    /// The `lint` subcommand
    #[options(help = "lint Advisory DB and ensure is well-formed")]
    Lint(LintCmd),

    /// The `web` subcommand
    #[options(help = "render advisory Markdown files for the rustsec.org web site")]
    Web(WebCmd),

    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),

    /// The `assign-id` subcommand
    #[options(help = "assigning RUSTSEC ids to new vulnerabilities")]
    AssignId(AssignIdCmd),

    /// The `osv` subcommand
    #[options(help = "export advisories to OSV format")]
    Osv(OsvCmd),

    /// The `ghsa` subcommand
    #[options(help = "import advisories from GHSA")]
    Ghsa(GhsaCmd),

    /// The `version` subcommand
    #[options(help = "list affected crate versions")]
    ListAffectedVersions(ListAffectedVersionsCmd),
}

impl Configurable<AppConfig> for AdminCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
