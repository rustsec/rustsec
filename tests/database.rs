use std::path::Path;
use cargo_lock::Lockfile;
use rustsec::Database;
use rustsec::database::package_scope::PackageScope;
use rustsec::database::Query;

/// Queries vulnerabilites in public package scope
#[test]
fn vulnerabilities_default() {
    let lockfile_path = Path::new("./tests/support/cratesio_cargo.lock");
    let lockfile = Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = Database::fetch().expect("Should find a default database.");
    let vuln = db.vulnerabilities(&lockfile);
    assert_eq!(vuln.len(), 1);
}

/// all package scope should be default
#[test]
fn query_vulnerabilities_default() {
    let lockfile_path = Path::new("./tests/support/cratesio_cargo.lock");
    let lockfile = Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = Database::fetch().expect("Should find a default database.");
    let vuln_all = db.query_vulnerabilities(&lockfile, &Query::crate_scope(), &PackageScope::All);
    let vuln = db.vulnerabilities(&lockfile);
    assert_eq!(vuln_all, vuln);
}

/// packages without source should not be queried in `PackageScope::LocalCrates` but in `PackageScope::PublicCrates`
#[test]
fn query_vulnerabilities_scope_local() {
    let lockfile_path = Path::new("./tests/support/local_cargo.lock");
    let lockfile = Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = Database::fetch().expect("Should find a default database.");
    let vuln_local = db.query_vulnerabilities(&lockfile, &Query::crate_scope(), &PackageScope::LocalCrates);
    assert_eq!(vuln_local.len(), 0);

    let vuln_public = db.query_vulnerabilities(&lockfile, &Query::crate_scope(), &PackageScope::PublicCrates);
    assert_eq!(vuln_public.len(), 1);
}