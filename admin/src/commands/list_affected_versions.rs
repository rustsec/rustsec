//! `rustsec-admin list-affected-versions` subcommand
//!
//! Can be used to verify that the version specification in the advisory
//! had the desired effect and matches only the versions you want it to


#![allow(clippy::never_loop)]

use abscissa_core::{Command, Options, Runnable};

/// `rustsec-admin list-affected-versions` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct ListAffectedVersionsCmd {}

impl Runnable for ListAffectedVersionsCmd {
    fn run(&self) {
        println!("hello world");
    }
}
