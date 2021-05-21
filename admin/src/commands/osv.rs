//! `rustsec-admin osv` subcommand

use abscissa_core::{Command, Options, Runnable};
#[derive(Command, Debug, Default, Options)]
pub struct OsvCmd {}

impl Runnable for OsvCmd {
    fn run(&self) {
        println!("hello world");
    }
}
