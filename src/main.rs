//! Audit Cargo.lock files for crates containing security vulnerabilities

#![crate_name = "cargo_audit"]
#![crate_type = "bin"]
#![deny(unsafe_code, warnings, missing_docs, trivial_numeric_casts)]
#![deny(trivial_casts, unused_import_braces, unused_qualifications)]

#[macro_use]
mod shell;

extern crate gumdrop;
#[allow(unused_imports)]
#[macro_use]
extern crate gumdrop_derive;
extern crate atty;
#[macro_use]
extern crate lazy_static;
extern crate platforms;
extern crate rustsec;
extern crate term;
#[macro_use]
extern crate serde_json;

use gumdrop::Options;
use platforms::target::{Arch, OS};
use rustsec::{
    Advisory, AdvisoryDatabase, ErrorKind, Lockfile, Package, Repository, Vulnerabilities,
    Vulnerability, ADVISORY_DB_REPO_URL,
};
use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};

/// Name of `Cargo.lock`
const CARGO_LOCK_FILE: &str = "Cargo.lock";

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

    /// Filesystem path to the advisory database git repository
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
    file: Option<String>,

    /// Allow stale advisory databases that haven't been recently updated
    #[options(no_short, long = "stale", help = "allow stale database")]
    stale: bool,

    /// Target CPU architecture to find vulnerabilities for
    #[options(
        no_short,
        long = "target-arch",
        help = "filter vulnerabilities by CPU (default: no filter)"
    )]
    target_arch: Option<Arch>,

    /// Target OS to find vulnerabilities for
    #[options(
        no_short,
        long = "target-os",
        help = "filter vulnerabilities by OS (default: no filter)"
    )]
    target_os: Option<OS>,

    /// URL to the advisory database git repository
    #[options(
        short = "u",
        long = "url",
        help = "URL for advisory database git repo"
    )]
    url: String,

    /// Output reports as JSON
    #[options(
        no_short,
        long = "json",
        help = "Output report in JSON format"
    )]
    output_json: bool,
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
            file: None,
            stale: false,
            target_arch: None,
            target_os: None,
            url: ADVISORY_DB_REPO_URL.into(),
            output_json: false,
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        help(1);
    }

    if args.len() > 2 {
        if args[2] == "help" || args[2] == "--help" {
            help(0);
        }

        if args[2] == "version" || args[2] == "--version" {
            version();
        }
    }

    let Opts::Audit(opts) = Opts::parse_args_default(&args[1..]).unwrap_or_else(|_| {
        help(1);
    });

    shell::init(&opts.color, use_stdout_for_status(&opts));

    audit(&opts, &load_advisory_db(&opts));
}

fn use_stdout_for_status(opts: &AuditOpts) -> bool {
    !opts.output_json
}

/// Print help message
fn help(code: i32) -> ! {
    println!("Usage: cargo audit [OPTIONS]");
    println!();
    println!("{}", Opts::command_usage("audit").unwrap());
    
    exit(code);
}

/// Print version message
fn version() -> ! {
    println!(concat!(
        env!("CARGO_PKG_NAME"),
        " ",
        env!("CARGO_PKG_VERSION")
    ));
    exit(0);
}

/// Load the advisory database
fn load_advisory_db(opts: &AuditOpts) -> AdvisoryDatabase {
    let advisory_repo_path = opts
        .db
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(Repository::default_path);

    status_ok!("Fetching", "advisory database from `{}`", opts.url);

    let advisory_db_repo = Repository::fetch(&opts.url, &advisory_repo_path, !opts.stale)
        .unwrap_or_else(|e| {
            status_error!("couldn't fetch advisory database: {}", e);
            exit(1);
        });

    let advisory_db = AdvisoryDatabase::from_repository(&advisory_db_repo).unwrap_or_else(|e| {
        status_error!("error loading advisory database: {}", e);
        exit(1);
    });

    status_ok!(
        "Loaded",
        "{} security advisories (from {})",
        advisory_db.advisories().count(),
        advisory_repo_path.display()
    );

    advisory_db
}

/// Run the audit operation
fn audit(opts: &AuditOpts, advisory_db: &AdvisoryDatabase) -> ! {
    let lockfile_path = opts
        .file
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(CARGO_LOCK_FILE));

    let lockfile = Lockfile::load(&lockfile_path).unwrap_or_else(|e| match e.kind() {
        ErrorKind::Io => {
            not_found(&lockfile_path);
            exit(1);
        }
        _ => {
            status_error!("Couldn't load {}: {}", lockfile_path.display(), e);
            exit(1);
        }
    });

    status_ok!(
        "Scanning",
        "{} for vulnerabilities ({} crate dependencies)",
        lockfile_path.display(),
        lockfile.packages.len(),
    );

    let all_matching_vulnerabilities = Vulnerabilities::find(&advisory_db, &lockfile);

    // TODO: factor affected platform checking upstream into `Vulnerabilities`
    let vulnerabilities: Vec<&Vulnerability> = all_matching_vulnerabilities
        .iter()
        .filter(|vuln| match_vulnerability(vuln, opts))
        .collect();

    if vulnerabilities.is_empty() {
        status_ok!("Success", "No vulnerable packages found");
        exit(0);
    }

    status_error!("Vulnerable crates found!");

    if opts.output_json {
        let json = json!({
            "database": {
                // TODO: AdvisoryDatabase does not know the Repository from which it was
                //       created. Making the Repository available in this scope would be
                //       of questionable value unless it was easily accessible.
                //"url": advisory_db,
                "advisory-count": advisory_db.advisories().count(),
            },
            "lockfile": {
                "path": lockfile_path,
                "dependency-count": lockfile.packages.len(),
            },
            "vulnerabilities": {
                "found": true,
                "count": vulnerabilities.len(),
                "list": vulnerabilities
            },
        });
        println!("{}", json.to_string());
    } else {
        for vuln in &vulnerabilities {
            display_advisory(&vuln.package, &vuln.advisory);
        }
    }

    if vulnerabilities.is_empty() {
        exit(0);
    } else {
        vulns_found(vulnerabilities.len());
        exit(1);
    }
}

/// Match a vulnerability according to the given audit options
fn match_vulnerability(vuln: &Vulnerability, opts: &AuditOpts) -> bool {
    if let Some(ref target_arch) = opts.target_arch {
        if let Some(ref architectures) = vuln.advisory.affected_arch {
            if !architectures.iter().any(|arch| arch == target_arch) {
                return false;
            }
        }
    }

    if let Some(ref target_os) = opts.target_os {
        if let Some(ref operating_systems) = vuln.advisory.affected_os {
            if !operating_systems.iter().any(|os| os == target_os) {
                return false;
            }
        }
    }

    true
}

fn not_found(path: &Path) {
    status_error!("Couldn't find '{}'!", path.display());
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
