use rustsec::{advisory, package, Database, Lockfile, Repository, VersionReq, DEFAULT_REPO_URL};
use tempfile::tempdir;

/// End-to-end integration test (has online dependency on GitHub)
#[test]
fn happy_path() {
    let db = Database::fetch().unwrap();
    let example_advisory_id = "RUSTSEC-2017-0001".parse::<advisory::Id>().unwrap();
    let example_advisory = db.find(&example_advisory_id).unwrap();
    let example_package = package::Name::from("sodiumoxide");

    assert_eq!(example_advisory.metadata.id, example_advisory_id);
    assert_eq!(example_advisory.metadata.package, example_package);
    assert_eq!(
        example_advisory.versions.patched[0],
        VersionReq::parse(">= 0.0.14").unwrap()
    );
    assert_eq!(example_advisory.metadata.date.as_str(), "2017-01-26");
    assert_eq!(
        example_advisory.metadata.url.as_ref().unwrap(),
        "https://github.com/dnaq/sodiumoxide/issues/154"
    );
    assert_eq!(
        example_advisory.metadata.title,
        "scalarmult() vulnerable to degenerate public keys"
    );
    assert_eq!(
        &example_advisory.metadata.description[0..30],
        "The `scalarmult()` function in"
    );

    let ref crate_advisories = db.find_by_crate(example_package);
    assert_eq!(example_advisory, crate_advisories[0]);

    let lockfile = Lockfile::load_file("Cargo.lock").unwrap();
    let vulns = db.vulnerabilities(&lockfile);
    assert!(vulns.is_empty());
}

/// Regression test for cloning into an existing directory
#[test]
fn clone_into_existing_directory() {
    // Make an empty temporary directory
    let tmp = tempdir().unwrap();

    // Attempt to fetch into it
    Repository::fetch(DEFAULT_REPO_URL, tmp.path(), true).unwrap();
}
