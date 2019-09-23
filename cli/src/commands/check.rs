//! RustSec Advisory DB Linter

use crate::prelude::*;
use abscissa_core::{Command, Runnable};
use gumdrop::Options;
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::exit,
};
use termcolor::{
    Color::{Green, Red},
    ColorChoice, ColorSpec, StandardStream, WriteColor,
};

macro_rules! writeln_color {
    ($stream:expr, $color:path, $fmt:expr, $msg:expr) => {
        let mut color = ColorSpec::new();
        color.bold();
        color.set_fg(Some($color));
        $stream.set_color(&color).unwrap();

        writeln!($stream, $fmt, $msg).unwrap();
        $stream.reset().unwrap();
    };
}

macro_rules! writeln_success {
    ($stream:expr, $msg:expr) => {
        writeln_color!($stream, Green, "✔ {}", $msg);
    };
    ($stream:expr, $fmt:expr, $($arg:tt)+) => {
        writeln_success!($stream, format!($fmt, $($arg)+));
    }
}

macro_rules! writeln_error {
    ($stream:expr, $msg:expr) => {
        writeln_color!($stream, Red, "✘ {}", $msg);
    };
    ($stream:expr, $fmt:expr, $($arg:tt)+) => {
        writeln_error!($stream, format!($fmt, $($arg)+));
    }
}

/// The `rustsec check` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct CheckCmd {
    /// Path to the advisory database
    #[options(free, help = "filesystem path to the RustSec advisory DB git repo")]
    path: Vec<PathBuf>,
}

impl Runnable for CheckCmd {
    fn run(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        let repo = rustsec::Repository::open(self.repo_path()).unwrap_or_else(|e| {
            status_err!(
                "couldn't open advisory DB repo from {}: {}",
                self.repo_path().display(),
                e
            );
            exit(1);
        });

        // Ensure Advisories.toml parses
        let db = rustsec::Database::load(&repo).unwrap();
        let advisories = db.iter();

        // Ensure we're parsing some advisories
        if advisories.len() == 0 {
            status_err!("no advisories found!");
            exit(1);
        }

        writeln_success!(
            &mut stdout,
            "Successfully parsed {} advisories",
            advisories.len()
        );

        let cratesio_client = crates_io_api::SyncClient::new();

        let mut invalid_advisories = 0;

        for advisory in advisories {
            if !self.check_advisory(&mut stdout, &cratesio_client, advisory) {
                invalid_advisories += 1;
            }
        }

        if invalid_advisories == 0 {
            writeln_success!(&mut stdout, "All advisories are well-formed");
        } else {
            writeln_error!(
                &mut stdout,
                "{} advisories contain errors!",
                invalid_advisories
            );
            exit(1);
        }
    }
}

impl CheckCmd {
    /// Get the path to the path to the RustSec Advisory DB get repository
    fn repo_path(&self) -> &Path {
        match self.path.len() {
            0 => Path::new("."),
            1 => self.path[0].as_path(),
            _ => Self::print_usage_and_exit(&[]),
        }
    }

    fn check_advisory(
        &self,
        stdout: &mut StandardStream,
        cratesio_client: &crates_io_api::SyncClient,
        advisory: &rustsec::Advisory,
    ) -> bool {
        if advisory.metadata.collection == Some(rustsec::Collection::Crates) {
            match cratesio_client.get_crate(advisory.metadata.package.as_str()) {
                Ok(response) => {
                    if response.crate_data.name != advisory.metadata.package.as_str() {
                        writeln_error!(
                            stdout,
                            "crates.io package name does not match package name in advisory for {}",
                            advisory.metadata.package.as_str()
                        );
                        return false;
                    }
                }
                Err(err) => {
                    writeln_error!(
                        stdout,
                        "Failed to get package `{}` from crates.io: {}",
                        advisory.metadata.package.as_str(),
                        err
                    );
                    return false;
                }
            }
        }

        let mut advisory_path = self
            .repo_path()
            .join(advisory.metadata.collection.as_ref().unwrap().to_string())
            .join(advisory.metadata.package.as_str())
            .join(advisory.metadata.id.as_str());

        advisory_path.set_extension("toml");

        let lint = rustsec::advisory::Linter::lint_file(&advisory_path).unwrap();

        if lint.errors().is_empty() {
            writeln_success!(
                stdout,
                "{} successfully passed lint",
                advisory_path.display()
            );
            true
        } else {
            writeln_error!(
                stdout,
                "{} contained the following lint errors:",
                advisory_path.display()
            );

            for error in lint.errors() {
                writeln!(stdout, "  - {}", error).unwrap();
            }

            false
        }
    }
}
