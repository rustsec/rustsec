//! `rustsec-admin version` subcommand

#![allow(clippy::never_loop)]

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// `rustsec-admin version` subcommand
#[derive(Command, Debug, Default, Parser)]
#[command(author, version, about)]
pub struct VersionCmd {}

impl Runnable for VersionCmd {
    /// Print version message
    fn run(&self) {
        println!("rustsec-admin {}", env!("CARGO_PKG_VERSION"));
    }
}
