//! `rustsec-admin assign-id` subcommand
//!
//! Assigns RUSTSEC ids to new vulnerabilities

use abscissa_core::{Command, Runnable};
use gumdrop::Options;
use std::path::{Path, PathBuf};

/// `rustsec-admin assign-id` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct AssignIdCmd {
    #[options(long = "github-actions-output")]
    github_action_output: bool,
    /// Path to the advisory database
    #[options(free, help = "filesystem path to the RustSec advisory DB git repo")]
    path: Vec<PathBuf>,
}

impl Runnable for AssignIdCmd {
    fn run(&self) {
        let repo_path = match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => Self::print_usage_and_exit(&[]),
        };
        let output_mode = if self.github_action_output {
            crate::assigner::OutputMode::GithubAction
        } else {
            crate::assigner::OutputMode::HumanReadable
        };

        crate::assigner::assign_ids(repo_path, output_mode);
    }
}
