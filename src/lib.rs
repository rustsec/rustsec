//! rustsec: Client library for the `RustSec` security advisory database

#![crate_name = "rustsec"]
#![crate_type = "lib"]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub mod advisory;
pub mod db;
pub mod error;
pub mod lockfile;
mod util;

extern crate reqwest;
extern crate semver;
extern crate toml;

pub use db::AdvisoryDatabase;
pub use lockfile::Lockfile;

/// URL where the TOML file containing the advisory database is located
pub const ADVISORY_DB_URL: &str =
    "https://raw.githubusercontent.com/RustSec/advisory-db/master/Advisories.toml";

#[cfg(test)]
mod tests {
    use db::AdvisoryDatabase;
    use lockfile::Lockfile;
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
}
