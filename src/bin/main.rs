//! The `cargo lock` subcommand

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unused_qualifications)]

use cargo_lock::{Lockfile, ResolveVersion};
use gumdrop::Options;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
};

/// Wrapper toplevel command for the `cargo lock` subcommand
#[derive(Options)]
enum CargoLock {
    #[options(help = "the `cargo lock` Cargo subcommand")]
    Lock(Command),
}

#[derive(Debug, Options)]
enum Command {
    #[options(help = "translate a Cargo.toml file")]
    Translate(TranslateCmd),
}

#[derive(Debug, Options)]
struct TranslateCmd {
    /// Input `Cargo.lock` file
    #[options(short = "f", help = "input Cargo.lock file to translate")]
    file: Option<PathBuf>,

    /// Output `Cargo.lock` file
    #[options(short = "o", help = "output Cargo.lock file (default STDOUT)")]
    output: Option<PathBuf>,

    /// Cargo.lock format version to translate to
    #[options(short = "v", help = "Cargo.lock resolve version to output")]
    version: Option<ResolveVersion>,
}

impl TranslateCmd {
    /// Translate `Cargo.lock` to a different format version
    pub fn run(&self) {
        let input = self
            .file
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or_else(|| Path::new("Cargo.lock"));

        let output = self
            .output
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or_else(|| Path::new("-"));

        let mut lockfile = Lockfile::load(input).unwrap_or_else(|e| {
            eprintln!("*** error: {}", e);
            exit(1);
        });

        lockfile.version = self.version.unwrap_or_default();
        let lockfile_toml = lockfile.to_string();

        if output == Path::new("-") {
            println!("{}", &lockfile_toml);
        } else {
            fs::write(output, lockfile_toml.as_bytes()).unwrap_or_else(|e| {
                eprintln!("*** error: {}", e);
                exit(1);
            });
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    match CargoLock::parse_args_default(&args[1..]) {
        Ok(CargoLock::Lock(cmd)) => match cmd {
            Command::Translate(translate) => translate.run(),
        },
        Err(e) => {
            eprintln!("*** error: {}", e);
            eprintln!("USAGE:");
            eprintln!("{}", Command::usage());
            exit(1);
        }
    }
}
