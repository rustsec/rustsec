#![cfg(feature = "fetch")]

use cargo_lock::Lockfile;
use once_cell::sync::Lazy;
use rustsec::database::scope;
use rustsec::database::Query;
use rustsec::repository::git::GitRepository;
use rustsec::Database;
use std::path::Path;
use std::sync::Mutex;

static DEFAULT_DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| {
    Mutex::new(
        Database::load(&GitRepository::fetch_default_repo().unwrap())
            .expect("Should be fetchable."),
    )
});

/// Queries vulnerabilites in public package scope
#[test]
fn vulnerabilities_default() {
    let lockfile_path = Path::new("./tests/support/cratesio_cargo.lock");
    let lockfile =
        Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = DEFAULT_DATABASE.lock().unwrap();
    let vuln = db.vulnerabilities(&lockfile);
    assert_eq!(vuln.len(), 1);
}

/// all package scope should be default
#[test]
fn query_vulnerabilities_default() {
    let lockfile_path = Path::new("./tests/support/cratesio_cargo.lock");
    let lockfile =
        Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = DEFAULT_DATABASE.lock().unwrap();
    let vuln_all =
        db.query_vulnerabilities(&lockfile, &Query::crate_scope(), scope::Package::default());
    let vuln = db.vulnerabilities(&lockfile);
    assert_eq!(vuln_all, vuln);
}

/// packages without source should not be queried in `package::Scope::LocalCrates` but in `PackageScope::PublicCrates`
#[test]
fn query_vulnerabilities_scope_public() {
    let lockfile_path = Path::new("./tests/support/local_cargo.lock");
    let lockfile =
        Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = DEFAULT_DATABASE.lock().unwrap();

    let vuln_public =
        db.query_vulnerabilities(&lockfile, &Query::crate_scope(), scope::Registry::Public);
    assert_eq!(vuln_public.len(), 0);

    let vuln_all = db.query_vulnerabilities(&lockfile, &Query::crate_scope(), scope::Registry::All);
    assert_eq!(vuln_all.len(), 1);
}
