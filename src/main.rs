//! Audit Cargo.lock files for crates containing security vulnerabilities

#![crate_name = "cargo_audit"]
#![crate_type = "bin"]
#![deny(unsafe_code, warnings, missing_docs, trivial_numeric_casts)]
#![deny(trivial_casts, unused_import_braces, unused_qualifications)]

#[macro_use]
mod shell;

extern crate gumdrop;
#[macro_use]
extern crate gumdrop_derive;
extern crate isatty;
#[macro_use]
extern crate lazy_static;
extern crate rustsec;
extern crate term;

use gumdrop::Options;
use rustsec::{
    Advisory, AdvisoryDatabase, ErrorKind, Lockfile, Package, Repository, Vulnerabilities,
    ADVISORY_DB_REPO_URL,
};
use std::{env, path::PathBuf, process::exit};

/// Command line arguments (parsed by gumdrop)
#[derive(Debug, Options)]
enum Opts {
    #[options(help = "Audit Cargo.lock files for vulnerable crates")]
    Audit(AuditOpts),
}

/// Options for the `cargo audit` subcommand
#[derive(Debug, Options)]
struct AuditOpts {
    /// Colored output configuration
    #[options(
        short = "c",
        long = "color",
        help = "color configuration: always, never (default: auto)"
    )]
    color: String,

    /// Path to the advisory database git repository
    #[options(
        short = "D",
        long = "db",
        help = "advisory database git repo path (default: ~/.cargo/advisory-db)"
    )]
    db: Option<String>,

    /// Path to the advisory database git repository
    #[options(
        short = "f",
        long = "file",
        help = "Cargo lockfile to inspect (default: Cargo.lock)"
    )]
    file: String,

    /// Allow stale advisory databases that haven't been recently updated
    #[options(no_short, long = "stale", help = "allow stale database")]
    stale: bool,

    /// URL to the advisory database git repository
    #[options(
        short = "u",
        long = "url",
        help = "URL for advisory database git repo"
    )]
    url: String,
}

/// Options for the `help` command
#[derive(Debug, Default, Options)]
struct HelpOpts {
    #[options(free)]
    commands: Vec<String>,
}

impl Default for AuditOpts {
    fn default() -> AuditOpts {
        AuditOpts {
            color: "auto".into(),
            db: None,
            file: "Cargo.lock".into(),
            stale: false,
            url: ADVISORY_DB_REPO_URL.into(),
        }
    }
}

impl AuditOpts {
    /// Run the audit operation
    fn call(&self) {
        shell::init(&self.color);

        let lockfile = Lockfile::load(&self.file).unwrap_or_else(|e| match e.kind() {
            ErrorKind::Io => {
                not_found(&self.file);
                exit(1);
            }
            _ => panic!("Couldn't load {}: {}", &self.file, e),
        });

        status_ok!("Fetching", "advisory database from `{}`", &self.url);

        let advisory_db = self
            .db
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(Repository::default_path);

        let repo = Repository::fetch(&self.url, &advisory_db, !self.stale).unwrap_or_else(|e| {
            status_error!("couldn't fetch advisory database: {}", e);
            exit(1);
        });

        let advisory_db = AdvisoryDatabase::from_repository(&repo).unwrap_or_else(|e| {
            status_error!("error loading advisory database: {}", e);
            exit(1);
        });

        status_ok!(
            "Scanning",
            "{} crates for vulnerabilities ({} advisories in database)",
            lockfile.packages.len(),
            advisory_db.advisories().count()
        );

        let vulnerabilities = Vulnerabilities::find(&advisory_db, &lockfile);

        if vulnerabilities.is_empty() {
            status_ok!("Success", "No vulnerable packages found");
            exit(0);
        }

        status_error!("Vulnerable crates found!");

        for vuln in &vulnerabilities {
            display_advisory(&vuln.package, &vuln.advisory);
        }

        if !vulnerabilities.is_empty() {
            vulns_found(vulnerabilities.len());
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        help();
    }

    if args.len() > 2 {
        if args[2] == "help" || args[2] == "--help" {
            help();
        }

        if args[2] == "version" || args[2] == "--version" {
            version();
        }
    }

    let Opts::Audit(opts) = Opts::parse_args_default(&args[1..]).unwrap_or_else(|_| {
        help();
    });

    opts.call();
}

/// Print help message
fn help() -> ! {
    println!("Usage: cargo audit [OPTIONS]");
    println!();
    println!("{}", Opts::command_usage("audit").unwrap());

    exit(2);
}

/// Print version message
fn version() -> ! {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    exit(2);
}

fn not_found(filename: &str) {
    status_error!("Couldn't find '{}'!", filename);
    println!("\nRun \"cargo build\" to generate lockfile before running audit");
}

fn vulns_found(vuln_count: usize) {
    println!();

    if vuln_count == 1 {
        status_error!("1 vulnerability found!");
    } else {
        status_error!("{} vulnerabilities found!", vuln_count);
    }
}

fn display_advisory(package: &Package, advisory: &Advisory) {
    attribute!("\nID", advisory.id.as_str());
    attribute!("Crate", package.name.as_str());
    attribute!("Version", &package.version.to_string());
    attribute!("Date", advisory.date.as_str());

    if let Some(url) = advisory.url.as_ref() {
        attribute!("URL", url);
    }

    attribute!("Title", &advisory.title);
    attribute!(
        "Solution: upgrade to",
        "{}",
        advisory
            .patched_versions
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .as_slice()
            .join(" OR ")
    );
}
