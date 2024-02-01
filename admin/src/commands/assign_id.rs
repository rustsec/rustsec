//! `rustsec-admin assign-id` subcommand
//!
//! Assigns RUSTSEC ids to new vulnerabilities

use abscissa_core::{Command, Runnable};
use clap::Parser;
use std::path::{Path, PathBuf};

/// `rustsec-admin assign-id` subcommand
#[derive(Command, Debug, Default, Parser)]
pub struct AssignIdCmd {
    #[arg(long = "github-actions-output")]
    github_action_output: bool,
    /// Path to the advisory database
    #[arg(
        num_args = 1..,
        help = "filesystem path to the RustSec advisory DB git repo"
    )]
    path: Vec<PathBuf>,
}

impl Runnable for AssignIdCmd {
    fn run(&self) {
        let repo_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => unreachable!(),
        };
        let output_mode = if self.github_action_output {
            crate::assigner::OutputMode::GithubAction
        } else {
            crate::assigner::OutputMode::HumanReadable
        };

        crate::assigner::assign_ids(repo_path, output_mode);
    }
}
