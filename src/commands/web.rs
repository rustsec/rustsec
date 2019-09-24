//! `rustsec-admin web`: Renderer for RustSec Advisory DB web site:
//!
//! https://rustsec.org

use abscissa_core::{Command, Runnable};
use gumdrop::Options;

/// `rustsec-admin web` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct WebCmd {}

impl Runnable for WebCmd {
    fn run(&self) {
        crate::web::render_advisories();
    }
}
