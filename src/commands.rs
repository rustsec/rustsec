//! `rustsec-admin` CLI subcommands

mod assignid;
mod lint;
mod version;
mod web;

use self::{assignid::AssignIdCmd, lint::LintCmd, version::VersionCmd, web::WebCmd};
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
}

impl Configurable<AppConfig> for AdminCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
