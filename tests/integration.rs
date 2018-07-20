extern crate rustsec;
extern crate semver;

use rustsec::{AdvisoryDatabase, Lockfile};
use semver::VersionReq;

// End-to-end integration test (has online dependency on GitHub)
#[test]
fn test_integration() {
    let db = AdvisoryDatabase::fetch().unwrap();
    let ref example_advisory = db.find("RUSTSEC-2017-0001").unwrap();

    assert_eq!(example_advisory.id, "RUSTSEC-2017-0001");
    assert_eq!(example_advisory.package, "sodiumoxide");
    assert_eq!(
        example_advisory.patched_versions[0],
        VersionReq::parse(">= 0.0.14").unwrap()
    );
    assert_eq!(example_advisory.date, Some(String::from("2017-01-26")));
    assert_eq!(
        example_advisory.url,
        Some(String::from(
            "https://github.com/dnaq/sodiumoxide/issues/154"
        ))
    );
    assert_eq!(
        example_advisory.title,
        "scalarmult() vulnerable to degenerate public keys"
    );
    assert_eq!(
        &example_advisory.description[0..30],
        "The `scalarmult()` function in"
    );

    let ref crate_advisories = db.find_by_crate("sodiumoxide");
    assert_eq!(*example_advisory, crate_advisories[0]);

    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    lockfile.vulnerabilities(&db);
}
