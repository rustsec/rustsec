//! `rustsec-admin web`: Renderer for RustSec Advisory DB web site:
//!
//! <https://rustsec.org>

use std::path::PathBuf;

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// `rustsec-admin web` subcommand
#[derive(Command, Debug, Default, Parser)]
pub struct WebCmd {
    #[arg(
        num_args = 1..,
        help = "path to output the generated website (defaults to _site/)"
    )]
    path: Vec<PathBuf>,
}

impl Runnable for WebCmd {
    fn run(&self) {
        let output_folder = match self.path.len() {
            0 => PathBuf::from("_site/"),
            1 => self.path[0].clone(),
            _ => unreachable!(),
        };
        crate::web::render_advisories(output_folder);
    }
}
