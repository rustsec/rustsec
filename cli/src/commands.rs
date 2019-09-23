//! `rustsec` CLI subcommands

mod check;
mod version;

use self::{check::CheckCmd, version::VersionCmd};
use crate::config::AppConfig;
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;

/// `rustsec` CLI subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum RustsecCliCmd {
    /// The `check` subcommand
    #[options(help = "check that the Advisory DB is well-formed")]
    Check(CheckCmd),

    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),
}

impl Configurable<AppConfig> for RustsecCliCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}
