//! `rustsec-admin` CLI subcommands

mod check;
mod version;
mod web;

use self::{check::CheckCmd, version::VersionCmd, web::WebCmd};
use crate::config::AppConfig;
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;

/// `rustsec-admin` CLI subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum AdminCmd {
    /// The `check` subcommand
    #[options(help = "check that the Advisory DB is well-formed")]
    Check(CheckCmd),

    /// The `web` subcommand
    #[options(help = "render advisory Markdown files for the rustsec.org web site")]
    Web(WebCmd),

    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),
}

impl Configurable<AppConfig> for AdminCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
