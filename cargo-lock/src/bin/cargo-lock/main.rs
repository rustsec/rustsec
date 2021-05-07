//! The `cargo lock` subcommand

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, unused_qualifications)]

use cargo_lock::{
    dependency::graph::EdgeDirection,
    dependency::Tree,
    package::{self},
    Dependency, Lockfile, ResolveVersion,
};
use gumdrop::Options;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::exit,
};

/// `cargo lock` subcommands
#[derive(Debug, Options)]
enum Command {
    /// The `cargo lock list` subcommand
    #[options(help = "list packages in Cargo.lock")]
    List(ListCmd),

    /// The `cargo lock translate` subcommand
    #[options(help = "translate a Cargo.lock file")]
    Translate(TranslateCmd),

    /// The `cargo lock tree` subcommand
    #[options(help = "print a dependency tree for the given dependency")]
    Tree(TreeCmd),
}

/// The `cargo lock list` subcommand
#[derive(Debug, Default, Options)]
struct ListCmd {
    /// Input `Cargo.lock` file
    #[options(short = "f", help = "input Cargo.lock file")]
    file: Option<PathBuf>,

    /// Get information for a specific package
    #[options(short = "p", help = "get information for a single package")]
    package: Option<package::Name>,

    /// List dependencies as part of the output
    #[options(short = "d", help = "show dependencies for each package")]
    dependencies: bool,

    /// Show package sources in list
    #[options(short = "s", help = "show package sources in listing")]
    sources: bool,
}

impl ListCmd {
    /// Display dependency summary from `Cargo.lock`
    pub fn run(&self) {
        for package in &load_lockfile(&self.file).packages {
            if let Some(name) = &self.package {
                if &package.name != name {
                    continue;
                }
            }

            if self.sources {
                println!("- {}", Dependency::from(package));
            } else {
                println!("- {} {}", package.name, package.version);
            }

            if self.dependencies {
                for dep in &package.dependencies {
                    if self.sources {
                        println!("  - {}", dep);
                    } else {
                        println!("  - {} {}", dep.name, dep.version);
                    }
                }
            }
        }
    }
}

/// The `cargo lock translate` subcommand
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
        let output = self
            .output
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or_else(|| Path::new("-"));

        let mut lockfile = load_lockfile(&self.file);

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

/// The `cargo lock tree` subcommand
#[derive(Debug, Options)]
struct TreeCmd {
    /// Input `Cargo.lock` file
    #[options(short = "f", help = "input Cargo.lock file to translate")]
    file: Option<PathBuf>,

    /// Dependencies names to draw a tree for
    #[options(free, help = "dependency names to draw trees for")]
    dependencies: Vec<package::Name>,
}

impl TreeCmd {
    /// Display dependency trees from `Cargo.lock`
    pub fn run(&self) {
        let lockfile = load_lockfile(&self.file);

        let tree = lockfile.dependency_tree().unwrap_or_else(|e| {
            eprintln!("*** error: {}", e);
            exit(1);
        });

        if self.dependencies.is_empty() {
            self.dependency_tree(&tree);
        } else {
            self.inverse_dependency_tree(&lockfile, &tree);
        }
    }

    /// Show forward dependency tree for detected root dependencies
    fn dependency_tree(&self, tree: &Tree) {
        for (i, index) in tree.roots().iter().enumerate() {
            if i > 0 {
                println!();
            }

            tree.render(&mut io::stdout(), *index, EdgeDirection::Outgoing)
                .unwrap();
        }
    }

    /// Show inverse dependency tree for the provided dependencies
    fn inverse_dependency_tree(&self, lockfile: &Lockfile, tree: &Tree) {
        for (i, dep) in self.dependencies.iter().enumerate() {
            if i > 0 {
                println!();
            }

            let package = lockfile
                .packages
                .iter()
                .find(|pkg| pkg.name == *dep)
                .unwrap_or_else(|| {
                    eprintln!("*** error: invalid dependency name: `{}`", dep);
                    exit(1);
                });

            let index = tree.nodes()[&package.into()];
            tree.render(&mut io::stdout(), index, EdgeDirection::Incoming)
                .unwrap();
        }
    }
}

/// Load a lockfile from the given path (or `Cargo.toml`)
fn load_lockfile(path: &Option<PathBuf>) -> Lockfile {
    let path = path
        .as_ref()
        .map(AsRef::as_ref)
        .unwrap_or_else(|| Path::new("Cargo.lock"));

    Lockfile::load(path).unwrap_or_else(|e| {
        eprintln!("*** error: {}", e);
        exit(1);
    })
}

fn main() {
    let mut args = env::args().collect::<Vec<_>>();

    // Remove leading arguments (bin and potential `lock`)
    if !args.is_empty() {
        args.remove(0);

        if args.get(0).map(AsRef::as_ref) == Some("lock") {
            args.remove(0);
        }
    }

    // If no command is specified, implicitly assume `list`
    if args.is_empty() || args[0].starts_with('-') {
        ListCmd::parse_args_default(&args)
            .unwrap_or_else(|e| {
                eprintln!("*** error: {}", e);
                eprintln!("USAGE:");
                eprintln!("{}", ListCmd::usage());
                exit(1);
            })
            .run();
        exit(0);
    }

    // ...otherwise parse and run the subcommand
    let cmd = Command::parse_args_default(&args).unwrap_or_else(|e| {
        eprintln!("*** error: {}", e);
        eprintln!("USAGE:");
        eprintln!("{}", Command::usage());
        exit(1);
    });

    match cmd {
        Command::List(list) => list.run(),
        Command::Translate(translate) => translate.run(),
        Command::Tree(tree) => tree.run(),
    }
}
