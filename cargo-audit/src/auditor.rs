//! Core auditing functionality

use crate::{config::AuditConfig, lockfile, prelude::*, presenter::Presenter};
use rustsec::{registry, report, Error, ErrorKind, Lockfile, Warning, WarningKind};
use std::{
    collections::btree_map as map,
    io::{self, Read},
    path::Path,
    process::exit,
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
            .unwrap_or(rustsec::repository::git::DEFAULT_URL);

        let advisory_db_path = config
            .database
            .path
            .as_ref()
            .cloned()
            .unwrap_or_else(rustsec::repository::git::Repository::default_path);

        let database = if config.database.fetch {
            if !config.output.is_quiet() {
                status_ok!("Fetching", "advisory database from `{}`", advisory_db_url);
            }

            let advisory_db_repo = rustsec::repository::git::Repository::fetch(
                advisory_db_url,
                &advisory_db_path,
                !config.database.stale,
            )
            .unwrap_or_else(|e| {
                status_err!("couldn't fetch advisory database: {}", e);
                exit(1);
            });

            rustsec::Database::load_from_repo(&advisory_db_repo).unwrap_or_else(|e| {
                status_err!("error loading advisory database: {}", e);
                exit(1);
            })
        } else {
            rustsec::Database::open(&advisory_db_path).unwrap_or_else(|e| {
                status_err!("error loading advisory database: {}", e);
                exit(1);
            })
        };

        if !config.output.is_quiet() {
            status_ok!(
                "Loaded",
                "{} security advisories (from {})",
                database.iter().count(),
                advisory_db_path.display()
            );
        }

        let registry_index = if config.yanked.enabled {
            if config.yanked.update_index && config.database.fetch {
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

    /// Perform an audit of a textual `Cargo.lock` file
    pub fn audit_lockfile(
        &mut self,
        maybe_lockfile_path: Option<&Path>,
    ) -> rustsec::Result<rustsec::Report> {
        let lockfile_path = match maybe_lockfile_path {
            Some(p) => p,
            None => {
                let path = Path::new(CARGO_LOCK_FILE);
                if !path.exists() && Path::new("Cargo.toml").exists() {
                    lockfile::generate()?;
                }
                path
            }
        };

        let lockfile = match self.load_lockfile(lockfile_path) {
            Ok(l) => l,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    &format!("Couldn't load {}: {}", lockfile_path.display(), e),
                ))
            }
        };

        self.presenter.before_report(lockfile_path, &lockfile);

        self.audit(&lockfile)
    }

    #[cfg(feature = "binary-scanning")]
    /// Perform an audit of multiple binary files with dependency data embedded by `cargo auditable`
    pub fn audit_binaries<P>(&mut self, binaries: &[P]) -> MultiFileReportSummmary
    where
        P: AsRef<Path>,
    {
        let mut summary = MultiFileReportSummmary::default();
        for path in binaries {
            let result = self.audit_binary(path.as_ref());
            match result {
                Ok(report) => {
                    if report.vulnerabilities.found {
                        summary.vulnerabilities_found = true;
                    }
                }
                Err(e) => {
                    status_err!("{}", e);
                    summary.errors_encountered = true;
                }
            }
        }
        summary
    }

    #[cfg(feature = "binary-scanning")]
    /// Perform an audit of a binary file with dependency data embedded by `cargo auditable`
    fn audit_binary(&mut self, binary_path: &Path) -> rustsec::Result<rustsec::Report> {
        self.presenter.before_binary_scan(binary_path);
        let lockfile = self.load_deps_from_binary(binary_path)?;
        self.audit(&lockfile)
    }

    /// The part of the auditing process that is shared between auditing lockfiles and binary files
    fn audit(&mut self, lockfile: &Lockfile) -> rustsec::Result<rustsec::Report> {
        let mut report = rustsec::Report::generate(&self.database, lockfile, &self.report_settings);

        // Warn for yanked crates
        if let Some(index) = &self.registry_index {
            if let Ok(yanked) = index.find_yanked(&lockfile.packages) {
                for pkg in yanked {
                    let warning = Warning::new(WarningKind::Yanked, pkg, None, None);

                    match report.warnings.entry(WarningKind::Yanked) {
                        map::Entry::Occupied(entry) => (*entry.into_mut()).push(warning),
                        map::Entry::Vacant(entry) => {
                            entry.insert(vec![warning]);
                        }
                    }
                }
            }
        }

        let self_advisories = self.self_advisories();

        self.presenter
            .print_report(&report, self_advisories.as_slice(), lockfile);

        Ok(report)
    }

    /// Load the lockfile to be audited
    fn load_lockfile(&self, lockfile_path: &Path) -> rustsec::Result<Lockfile> {
        if lockfile_path == Path::new("-") {
            // Read Cargo.lock from STDIN
            let mut lockfile_toml = String::new();
            io::stdin().read_to_string(&mut lockfile_toml)?;
            Ok(lockfile_toml.parse()?)
        } else {
            Ok(Lockfile::load(lockfile_path)?)
        }
    }

    #[cfg(feature = "binary-scanning")]
    /// Load the dependency tree from a binary file built with `cargo auditable`
    fn load_deps_from_binary(&self, binary_path: &Path) -> rustsec::Result<Lockfile> {
        // Read the input
        let binary = if binary_path == Path::new("-") {
            let mut input = Vec::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_end(&mut input)?;
            input
        } else {
            let f = std::fs::File::open(binary_path)?;
            let mut f = std::io::BufReader::new(f);
            let mut input = Vec::new();
            f.read_to_end(&mut input)?;
            input
        };
        // Extract the compressed audit data
        let compressed_audit_data = auditable_extract::raw_auditable_data(&binary).map_err(|e| {
            // The error handling boilerplate is in here instead of the `rustsec` crate because as of this writing
            // the public APIs of the crates involved are still somewhat unstable,
            // and this way we don't expose the error types in any public APIs
            match e {
                auditable_extract::Error::NoAuditData => Error::new(
                    ErrorKind::Parse,
                    &"No audit data found! Was the binary built with 'cargo auditable'?",
                ),
                other => Error::new(ErrorKind::Parse, &other)
            }
        })?;
        // Decompress with a 8MiB size limit. Audit data JSONs don't get that large; anything this big is a DoS attempt
        use miniz_oxide::inflate::decompress_to_vec_zlib_with_limit;
        let text = decompress_to_vec_zlib_with_limit(compressed_audit_data, 1024 * 1024 * 8).map_err(|e| {
            Error::new(ErrorKind::Parse, &e)
        })?;
        let json_structs: auditable_serde::VersionInfo = serde_json::from_slice(&text).map_err(|e| {
            Error::new(ErrorKind::Parse, &e)
        })?;
        let lockfile = cargo_lock::Lockfile::try_from(&json_structs)?;
        Ok(lockfile)
    }

    /// Query the database for advisories about `cargo-audit` or `rustsec` itself
    fn self_advisories(&mut self) -> Vec<rustsec::Advisory> {
        let mut results = vec![];

        for (package_name, package_version) in [
            ("cargo-audit", crate::VERSION),
            ("rustsec", rustsec::VERSION),
        ] {
            let query = rustsec::database::Query::crate_scope()
                .package_name(package_name.parse().unwrap())
                .package_version(package_version.parse().unwrap());

            for advisory in self.database.query(&query) {
                results.push(advisory.clone());
            }
        }

        results
    }
}

/// Summary of the report over multiple scanned files
#[derive(Clone, Copy, Debug, Default)]
pub struct MultiFileReportSummmary {
    /// Whether any vulnerabilities were found
    pub vulnerabilities_found: bool,
    /// Whether any errors were encountered during scanning
    pub errors_encountered: bool,
}
