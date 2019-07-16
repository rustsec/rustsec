//! The `cargo audit` subcommand

use crate::config::CargoAuditConfig;
use abscissa_core::{config::Override, Command, FrameworkError, Runnable};
use gumdrop::Options;
use platforms::target::{Arch, OS};
use rustsec::{
    Advisory, AdvisoryDatabase, ErrorKind, Lockfile, Package, Repository, Vulnerabilities,
    Vulnerability, ADVISORY_DB_REPO_URL,
};
use serde_json::json;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

/// Name of `Cargo.lock`
const CARGO_LOCK_FILE: &str = "Cargo.lock";

/// The `cargo audit` subcommand
#[derive(Command, Default, Debug, Options)]
pub struct AuditCommand {
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
    #[options(short = "u", long = "url", help = "URL for advisory database git repo")]
    url: Option<String>,

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
        let lockfile_path = self
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
                status_err!("Couldn't load {}: {}", lockfile_path.display(), e);
                exit(1);
            }
        });

        let advisory_db = self.load_advisory_db();

        status_ok!(
            "Scanning",
            "{} for vulnerabilities ({} crate dependencies)",
            lockfile_path.display(),
            lockfile.packages.len(),
        );

        let all_matching_vulnerabilities = Vulnerabilities::find(&advisory_db, &lockfile);

        // TODO: factor affected platform checking upstream into `Vulnerabilities`
        let vulnerabilities = all_matching_vulnerabilities
            .iter()
            .filter(|vuln| self.match_vulnerability(vuln))
            .collect::<Vec<_>>();

        if vulnerabilities.is_empty() {
            status_ok!("Success", "No vulnerable packages found");
        } else {
            status_err!("Vulnerable crates found!");
        }

        if self.output_json {
            let json = json!({
                "database": {
                    "advisory-count": advisory_db.advisories().count(),
                },
                "lockfile": {
                    "path": lockfile_path,
                    "dependency-count": lockfile.packages.len(),
                },
                "vulnerabilities": {
                    "found": !vulnerabilities.is_empty(),
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
}

impl AuditCommand {
    /// Load the advisory database
    fn load_advisory_db(&self) -> AdvisoryDatabase {
        let advisory_repo_url = self
            .url
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or(ADVISORY_DB_REPO_URL);

        let advisory_repo_path = self
            .db
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(Repository::default_path);

        status_ok!("Fetching", "advisory database from `{}`", advisory_repo_url);

        let advisory_db_repo =
            Repository::fetch(advisory_repo_url, &advisory_repo_path, !self.stale).unwrap_or_else(
                |e| {
                    status_err!("couldn't fetch advisory database: {}", e);
                    exit(1);
                },
            );

        let advisory_db =
            AdvisoryDatabase::from_repository(&advisory_db_repo).unwrap_or_else(|e| {
                status_err!("error loading advisory database: {}", e);
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

    /// Match a vulnerability according to the given audit options
    fn match_vulnerability(&self, vuln: &Vulnerability) -> bool {
        if let Some(ref target_arch) = self.target_arch {
            if let Some(ref architectures) = vuln.advisory.affected_arch {
                if !architectures.iter().any(|arch| arch == target_arch) {
                    return false;
                }
            }
        }

        if let Some(ref target_os) = self.target_os {
            if let Some(ref operating_systems) = vuln.advisory.affected_os {
                if !operating_systems.iter().any(|os| os == target_os) {
                    return false;
                }
            }
        }

        if self.ignore.contains(&vuln.advisory.id.as_str().to_owned()) {
            return false;
        }

        true
    }
}

fn not_found(path: &Path) {
    status_err!("Couldn't find '{}'!", path.display());
    println!("\nRun \"cargo generate-lockfile\" to generate lockfile before running audit");
}

fn vulns_found(vuln_count: usize) {
    println!();

    if vuln_count == 1 {
        status_err!("1 vulnerability found!");
    } else {
        status_err!("{} vulnerabilities found!", vuln_count);
    }
}

fn display_advisory(package: &Package, advisory: &Advisory) {
    status_attr_err!("\nID", advisory.id.as_str());
    status_attr_err!("Crate", package.name.as_str());
    status_attr_err!("Version", &package.version.to_string());
    status_attr_err!("Date", advisory.date.as_str());

    if let Some(url) = advisory.url.as_ref() {
        status_attr_err!("URL", url);
    }

    status_attr_err!("Title", &advisory.title);
    status_attr_err!(
        "Solution: upgrade to",
        "{}",
        advisory
            .patched_versions
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .as_slice()
            .join(" OR ")
    );
}
