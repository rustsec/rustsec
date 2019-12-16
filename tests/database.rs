use cargo_lock::Lockfile;
use rustsec::database::package_scope::{PackageScope, PackageSource};
use rustsec::database::Query;
use rustsec::Database;
use std::path::Path;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref DEFAULT_DATABASE: Mutex<Database> =
        { Mutex::new(Database::fetch().expect("Should be fetchable.")) };
}

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
        db.query_vulnerabilities(&lockfile, &Query::crate_scope(), &PackageScope::default());
    let vuln = db.vulnerabilities(&lockfile);
    assert_eq!(vuln_all, vuln);
}

/// packages without source should not be queried in `PackageScope::LocalCrates` but in `PackageScope::PublicCrates`
#[test]
fn query_vulnerabilities_scope_public() {
    let lockfile_path = Path::new("./tests/support/local_cargo.lock");
    let lockfile =
        Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = DEFAULT_DATABASE.lock().unwrap();

    let vuln_public = db.query_vulnerabilities(
        &lockfile,
        &Query::crate_scope(),
        &PackageScope::from_source(PackageSource::Public),
    );
    assert_eq!(vuln_public.len(), 0);

    let vuln_all = db.query_vulnerabilities(
        &lockfile,
        &Query::crate_scope(),
        &PackageScope::from_source(PackageSource::All),
    );
    assert_eq!(vuln_all.len(), 1);
}
