//! Core auditing functionality

use crate::{
    config::AuditConfig,
    error::{Error, ErrorKind},
    prelude::*,
    presenter::Presenter,
};
use rustsec::{lockfile::Lockfile, report};
use std::{
    io::{self, Read},
    path::Path,
    process::exit,
};

/// Security vulnerability auditor
pub struct Auditor {
    /// RustSec Advisory Database
    database: rustsec::Database,

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
            .unwrap_or(rustsec::DEFAULT_REPO_URL);

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

        if let Ok(support_info) = advisory_db_repo.support() {
            if let Some(rustsec_update) = support_info.rustsec.next_update {
                if !rustsec_update
                    .version
                    .matches(&rustsec::VERSION.parse().unwrap())
                {
                    status_warn!(
                        "support for this version of `cargo-audit` ends on {}. Please upgrade!",
                        rustsec_update.date.as_str()
                    );
                }
            }
        }

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

        Self {
            database,
            presenter: Presenter::new(&config.output),
            report_settings: config.report_settings(),
        }
    }

    /// Perform audit
    pub fn audit(&mut self, lockfile_path: &Path) -> rustsec::Report {
        let lockfile = self
            .load_lockfile(lockfile_path)
            .unwrap_or_else(|e| match e.kind() {
                ErrorKind::Io => {
                    status_err!("Couldn't find '{}'!", lockfile_path.display());
                    println!(
                    "\nRun \"cargo generate-lockfile\" to generate lockfile before running audit"
                );
                    exit(1);
                }
                _ => {
                    status_err!("Couldn't load {}: {}", lockfile_path.display(), e);
                    exit(1);
                }
            });

        self.presenter.before_report(&lockfile_path, &lockfile);

        let report = rustsec::Report::generate(&self.database, &lockfile, &self.report_settings);
        self.presenter.print_report(&report, &lockfile);
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
}
