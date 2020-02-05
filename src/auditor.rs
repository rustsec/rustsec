//! Core auditing functionality

use crate::{config::AuditConfig, error::Error, prelude::*, presenter::Presenter};
use rustsec::{lockfile::Lockfile, registry, report, warning, Warning};
use std::{
    io::{self, Read},
    path::Path,
    process::{exit, Command},
};

/// Name of `Cargo.lock`
const CARGO_LOCK_FILE: &str = "Cargo.lock";

/// Security vulnerability auditor
pub struct Auditor {
    /// RustSec Advisory Database
    database: rustsec::Database,

    /// Crates.io registry index
    registry_index: Option<registry::Index>,

    /// Presenter for displaying the report
    presenter: Presenter,

    /// Audit report settings
    report_settings: report::Settings,
}

impl Auditor {
    /// Initialize the auditor
    pub fn new(config: &AuditConfig) -> Self {
        let advisory_db_url = config
            .database
            .url
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or(rustsec::repository::DEFAULT_URL);

        let advisory_db_path = config
            .database
            .path
            .as_ref()
            .cloned()
            .unwrap_or_else(rustsec::Repository::default_path);

        let advisory_db_repo = if config.database.fetch {
            if !config.output.is_quiet() {
                status_ok!("Fetching", "advisory database from `{}`", advisory_db_url);
            }

            rustsec::Repository::fetch(advisory_db_url, &advisory_db_path, !config.database.stale)
                .unwrap_or_else(|e| {
                    status_err!("couldn't fetch advisory database: {}", e);
                    exit(1);
                })
        } else {
            rustsec::Repository::open(&advisory_db_path).unwrap_or_else(|e| {
                status_err!("couldn't open advisory database: {}", e);
                exit(1);
            })
        };

        let database = rustsec::Database::load(&advisory_db_repo).unwrap_or_else(|e| {
            status_err!("error loading advisory database: {}", e);
            exit(1);
        });

        if !config.output.is_quiet() {
            status_ok!(
                "Loaded",
                "{} security advisories (from {})",
                database.iter().count(),
                advisory_db_path.display()
            );
        }

        let registry_index = if config.yanked.enabled {
            if config.yanked.update_index {
                if !config.output.is_quiet() {
                    status_ok!("Updating", "crates.io index");
                }

                match registry::Index::fetch() {
                    Ok(index) => Some(index),
                    Err(err) => {
                        if !config.output.is_quiet() {
                            status_warn!("couldn't update crates.io index: {}", err);
                        }

                        None
                    }
                }
            } else {
                match registry::Index::open() {
                    Ok(index) => Some(index),
                    Err(err) => {
                        if !config.output.is_quiet() {
                            status_warn!("couldn't open crates.io index: {}", err);
                        }

                        None
                    }
                }
            }
        } else {
            None
        };

        Self {
            database,
            registry_index,
            presenter: Presenter::new(&config.output),
            report_settings: config.report_settings(),
        }
    }

    /// Perform audit
    pub fn audit(&mut self, maybe_lockfile_path: Option<&Path>) -> rustsec::Report {
        let lockfile_path = maybe_lockfile_path.unwrap_or_else(|| {
            let path = Path::new(CARGO_LOCK_FILE);

            if !path.exists() && Path::new("Cargo.toml").exists() {
                generate_lockfile();
            }

            path
        });

        let lockfile = self.load_lockfile(lockfile_path).unwrap_or_else(|e| {
            status_err!("Couldn't load {}: {}", lockfile_path.display(), e);
            exit(1);
        });

        self.presenter.before_report(&lockfile_path, &lockfile);

        let mut report =
            rustsec::Report::generate(&self.database, &lockfile, &self.report_settings);

        // Warn for yanked crates
        if let Some(index) = &self.registry_index {
            for package in &lockfile.packages {
                if let Ok(pkg) = index.find(&package.name, &package.version) {
                    if pkg.is_yanked {
                        let warning = Warning::new(warning::Kind::Yanked, package);
                        report.warnings.push(warning);
                    }
                }
            }
        }

        let self_advisories = self.self_advisories();

        self.presenter
            .print_report(&report, self_advisories.as_slice(), &lockfile);

        report
    }

    /// Load the lockfile to be audited
    fn load_lockfile(&self, lockfile_path: &Path) -> Result<Lockfile, Error> {
        if lockfile_path == Path::new("-") {
            // Read Cargo.lock from STDIN
            let mut lockfile_toml = String::new();
            io::stdin().read_to_string(&mut lockfile_toml)?;
            Ok(lockfile_toml.parse()?)
        } else {
            Ok(Lockfile::load(lockfile_path)?)
        }
    }

    /// Query the database for advisories about `cargo-audit` or `rustsec` itself
    fn self_advisories(&mut self) -> Vec<rustsec::Advisory> {
        let mut results = vec![];

        for (package_str, version_str) in &[
            ("cargo-audit", crate::VERSION),
            ("rustsec", rustsec::VERSION),
        ] {
            let package: rustsec::package::Name = package_str.parse().unwrap();
            let version: rustsec::Version = version_str.parse().unwrap();
            let query = rustsec::database::Query::crate_scope();

            for advisory in self
                .database
                .query(&query.package_version(package, version))
            {
                results.push(advisory.clone());
            }
        }

        results
    }
}

/// Run `cargo generate-lockfile`
fn generate_lockfile() {
    let status = Command::new("cargo")
        .arg("generate-lockfile")
        .status()
        .unwrap_or_else(|e| {
            status_err!("couldn't run `cargo generate-lockfile`: {}", e);
            exit(1);
        });

    if !status.success() {
        if let Some(code) = status.code() {
            status_err!(
                "non-zero exit status running `cargo generate-lockfile`: {}",
                code
            );
        } else {
            status_err!("no exit status running `cargo generate-lockfile`!");
        }

        exit(1);
    }
}
