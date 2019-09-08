//! The `cargo audit` subcommand

use super::CargoAuditCommand;
use crate::{
    config::CargoAuditConfig,
    error::{Error, ErrorKind},
    tree::Tree,
};
use abscissa_core::{
    config::Override,
    terminal::{
        self,
        Color::{Red, Yellow},
    },
    Command, FrameworkError, Runnable,
};
use gumdrop::Options;
use rustsec::{
    advisory,
    platforms::target::{Arch, OS},
    report, Lockfile, Vulnerability,
};
use std::{
    collections::BTreeSet as Set,
    io::{self, Read},
    path::{Path, PathBuf},
    process::exit,
};

/// Name of `Cargo.lock`
const CARGO_LOCK_FILE: &str = "Cargo.lock";

/// The `cargo audit` subcommand
#[derive(Command, Default, Debug, Options)]
pub struct AuditCommand {
    /// Version information
    #[options(no_short, long = "version", help = "output version and exit")]
    version: bool,

    /// Colored output configuration
    #[options(
        short = "c",
        long = "color",
        help = "color configuration: always, never (default: auto)"
    )]
    color: Option<String>,

    /// Filesystem path to the advisory database git repository
    #[options(
        short = "D",
        long = "db",
        help = "advisory database git repo path (default: ~/.cargo/advisory-db)"
    )]
    db: Option<String>,

    /// Path to the lockfile
    #[options(
        short = "f",
        long = "file",
        help = "Cargo lockfile to inspect (or `-` for STDIN, default: Cargo.lock)"
    )]
    file: Option<String>,

    /// Skip fetching the advisory database git repository
    #[options(
        short = "n",
        long = "no-fetch",
        help = "do not perform a git fetch on the advisory DB"
    )]
    no_fetch: bool,

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
    #[options(short = "u", long = "url", help = "URL for advisory database git repo")]
    url: Option<String>,

    /// Quiet mode - avoids printing extraneous information
    #[options(
        short = "q",
        long = "quiet",
        help = "Avoid printing unnecessary information"
    )]
    quiet: bool,

    /// Output reports as JSON
    #[options(no_short, long = "json", help = "Output report in JSON format")]
    output_json: bool,

    /// Advisory ids to ignore
    #[options(
        no_short,
        long = "ignore",
        meta = "ADVISORY_ID",
        help = "Advisory id to ignore (can be specified multiple times)"
    )]
    ignore: Vec<String>,
}

impl Override<CargoAuditConfig> for AuditCommand {
    fn override_config(
        &self,
        mut config: CargoAuditConfig,
    ) -> Result<CargoAuditConfig, FrameworkError> {
        if let Some(color) = &self.color {
            config.display.color = Some(color.clone());
        }

        Ok(config)
    }
}

impl Runnable for AuditCommand {
    fn run(&self) {
        if self.version {
            println!(
                "{} {}",
                CargoAuditCommand::name(),
                CargoAuditCommand::version()
            );
            exit(0);
        }

        let lockfile = self.load_lockfile().unwrap_or_else(|e| match e.kind() {
            ErrorKind::Io => {
                status_err!("Couldn't find '{}'!", self.lockfile_path().display());
                println!(
                    "\nRun \"cargo generate-lockfile\" to generate lockfile before running audit"
                );
                exit(1);
            }
            _ => {
                status_err!("Couldn't load {}: {}", self.lockfile_path().display(), e);
                exit(1);
            }
        });

        let advisory_db = self.load_advisory_db();

        if !self.quiet() {
            status_ok!(
                "Scanning",
                "{} for vulnerabilities ({} crate dependencies)",
                self.lockfile_path().display(),
                lockfile.packages.len(),
            );
        }

        let report = rustsec::Report::generate(&advisory_db, &lockfile, &self.report_settings());

        if !self.quiet() {
            if report.vulnerabilities.found {
                status_err!("Vulnerable crates found!");
            } else {
                status_ok!("Success", "No vulnerable packages found");
            }
        }

        if self.output_json {
            serde_json::to_writer(io::stdout(), &report).unwrap();
        } else {
            // Track packages we've displayed once so we don't show the same dep tree
            // TODO(tarcieri): group advisories about the same package?
            let mut displayed_packages = Set::new();

            for vulnerability in &report.vulnerabilities.list {
                if displayed_packages.insert(vulnerability.package.name.clone()) {
                    let tree = Tree::new(&lockfile, &vulnerability.package);
                    display_vulnerability(&vulnerability, Some(&tree));
                } else {
                    display_vulnerability(&vulnerability, None);
                }
            }

            if !report.warnings.is_empty() {
                println!();

                status_warn!("found informational advisories for dependencies");

                for warning in &report.warnings {
                    println!();

                    display_attr(Yellow, "Crate:   ", warning.package.as_str());
                    display_attr(Red, "Message: ", warning.message.as_str());

                    if let Some(url) = &warning.url {
                        display_attr(Yellow, "URL:     ", url);
                    }
                }
            }
        }

        if report.vulnerabilities.found {
            println!();

            if report.vulnerabilities.count == 1 {
                status_err!("1 vulnerability found!");
            } else {
                status_err!("{} vulnerabilities found!", report.vulnerabilities.count);
            }

            exit(1);
        } else {
            exit(0);
        }
    }
}

impl AuditCommand {
    /// Should we suppress excessive output?
    fn quiet(&self) -> bool {
        self.quiet || self.output_json
    }

    /// Load the advisory database
    fn load_advisory_db(&self) -> rustsec::Database {
        let advisory_repo_url = self
            .url
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or(rustsec::DEFAULT_REPO_URL);

        let advisory_repo_path = self
            .db
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(rustsec::Repository::default_path);

        let advisory_db_repo = if self.no_fetch {
            rustsec::Repository::open(&advisory_repo_path).unwrap_or_else(|e| {
                status_err!("couldn't open advisory database: {}", e);
                exit(1);
            })
        } else {
            if !self.quiet() {
                status_ok!("Fetching", "advisory database from `{}`", advisory_repo_url);
            }

            rustsec::Repository::fetch(advisory_repo_url, &advisory_repo_path, !self.stale)
                .unwrap_or_else(|e| {
                    status_err!("couldn't fetch advisory database: {}", e);
                    exit(1);
                })
        };

        let advisory_db = rustsec::Database::load(&advisory_db_repo).unwrap_or_else(|e| {
            status_err!("error loading advisory database: {}", e);
            exit(1);
        });

        if !self.quiet() {
            status_ok!(
                "Loaded",
                "{} security advisories (from {})",
                advisory_db.iter().count(),
                advisory_repo_path.display()
            );
        }

        advisory_db
    }

    /// Get the path to the lockfile
    fn lockfile_path(&self) -> PathBuf {
        self.file
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(CARGO_LOCK_FILE))
    }

    /// Load the lockfile to be audited
    fn load_lockfile(&self) -> Result<Lockfile, Error> {
        let path = self.lockfile_path();

        if path.as_path() == Path::new("-") {
            // Read Cargo.lock from STDIN
            let mut lockfile_toml = String::new();
            io::stdin().read_to_string(&mut lockfile_toml)?;
            Ok(lockfile_toml.parse()?)
        } else {
            Ok(Lockfile::load_file(path)?)
        }
    }

    /// Get the report settings to use
    fn report_settings(&self) -> report::Settings {
        let mut settings = rustsec::report::Settings::default();
        settings.target_arch = self.target_arch;
        settings.target_os = self.target_os;
        settings.severity = Some(advisory::Severity::Low);
        settings.informational_warnings = vec![advisory::Informational::Unmaintained];
        settings.ignore = self
            .ignore
            .iter()
            .map(|id| {
                id.parse().unwrap_or_else(|e| {
                    status_err!("error parsing {}: {}", id, e);
                    exit(1);
                })
            })
            .collect();

        settings
    }
}

/// Display information about a particular vulnerability
fn display_vulnerability(vulnerability: &Vulnerability, tree: Option<&Tree>) {
    let advisory = &vulnerability.advisory;

    println!();
    display_attr(Red, "ID:      ", advisory.id.as_str());
    display_attr(Red, "Crate:   ", vulnerability.package.name.as_str());
    display_attr(Red, "Version: ", &vulnerability.package.version.to_string());
    display_attr(Red, "Date:    ", advisory.date.as_str());

    if let Some(url) = advisory.url.as_ref() {
        display_attr(Red, "URL:     ", url);
    }

    display_attr(Red, "Title:   ", &advisory.title);
    display_attr(
        Red,
        "Solution: upgrade to",
        &vulnerability
            .versions
            .patched
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .as_slice()
            .join(" OR "),
    );

    if let Some(t) = tree {
        terminal::status::Status::new()
            .bold()
            .color(Red)
            .status("Dependency tree:")
            .print_stdout("")
            .unwrap();

        t.print();
    }
}

/// Display an attribute of a particular vulnerability
fn display_attr(color: terminal::Color, attr: &str, content: &str) {
    terminal::status::Status::new()
        .bold()
        .color(color)
        .status(attr)
        .print_stdout(content)
        .unwrap();
}
