//! Core auditing functionality

use crate::{
    config::{AuditConfig, OutputFormat},
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
    /// Are we operating in quiet mode?
    quiet: bool,

    /// Should dependency trees be displayed?
    show_dependency_tree: bool,

    /// RustSec Advisory Database
    database: rustsec::Database,

    /// Output format to display
    output_format: OutputFormat,

    /// Report settings to use when generating audit report
    report_settings: report::Settings,
}

impl Auditor {
    /// Initialize the auditor
    pub fn new(config: &AuditConfig) -> Self {
        // Use quiet mode if explicitly configured or if we're outputting JSON
        let quiet = config.quiet || config.output_format == OutputFormat::Json;
        let show_dependency_tree = config.show_dependency_tree.unwrap_or(true);

        let advisory_db_url = config
            .advisory_db_url
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or(rustsec::DEFAULT_REPO_URL);

        let advisory_db_path = config
            .advisory_db_path
            .as_ref()
            .cloned()
            .unwrap_or_else(rustsec::Repository::default_path);

        let advisory_db_repo = if config.no_fetch {
            rustsec::Repository::open(&advisory_db_path).unwrap_or_else(|e| {
                status_err!("couldn't open advisory database: {}", e);
                exit(1);
            })
        } else {
            if !quiet {
                status_ok!("Fetching", "advisory database from `{}`", advisory_db_url);
            }

            rustsec::Repository::fetch(advisory_db_url, &advisory_db_path, !config.allow_stale)
                .unwrap_or_else(|e| {
                    status_err!("couldn't fetch advisory database: {}", e);
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

        if !quiet {
            status_ok!(
                "Loaded",
                "{} security advisories (from {})",
                database.iter().count(),
                advisory_db_path.display()
            );
        }

        Self {
            show_dependency_tree,
            quiet,
            database,
            output_format: config.output_format,
            report_settings: config.report_settings(),
        }
    }

    /// Perform audit
    pub fn audit(&self, lockfile_path: &Path) {
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

        if !self.quiet {
            status_ok!(
                "Scanning",
                "{} for vulnerabilities ({} crate dependencies)",
                lockfile_path.display(),
                lockfile.packages.len(),
            );
        }

        let report = rustsec::Report::generate(&self.database, &lockfile, &self.report_settings);

        if !self.quiet {
            if report.vulnerabilities.found {
                status_err!("Vulnerable crates found!");
            } else {
                status_ok!("Success", "No vulnerable packages found");
            }
        }

        match self.output_format {
            OutputFormat::Json => serde_json::to_writer(io::stdout(), &report).unwrap(),
            OutputFormat::Terminal => {
                let mut presenter = Presenter::new(&lockfile, self.show_dependency_tree);

                for vulnerability in &report.vulnerabilities.list {
                    presenter.print_vulnerability(vulnerability);
                }

                if !report.warnings.is_empty() {
                    println!();
                    status_warn!("found informational advisories for dependencies");

                    for warning in &report.warnings {
                        presenter.print_warning(warning)
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
        }
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
