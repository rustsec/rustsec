//! Tests for parsing RustSec advisories

#![warn(rust_2018_idioms, unused_qualifications)]

use rustsec::{advisory::Severity, database::Query, package};

/// Load example advisory from the filesystem
fn load_advisory() -> rustsec::Advisory {
    rustsec::Advisory::load_file("./tests/support/example_advisory_v3.md").unwrap()
}

#[test]
fn matches_name() {
    let advisory = load_advisory();

    let package_matches: package::Name = "base".parse().unwrap();
    let query_matches = Query::new().package_name(package_matches);
    assert!(query_matches.matches(&advisory));

    let package_nomatch: package::Name = "somethingelse".parse().unwrap();
    let query_nomatch = Query::new().package_name(package_nomatch);
    assert!(!query_nomatch.matches(&advisory));
}

#[test]
fn matches_year() {
    let advisory = load_advisory();

    let query_matches = Query::new().year(2001);
    assert!(query_matches.matches(&advisory));

    let query_nomatch = Query::new().year(2525);
    assert!(!query_nomatch.matches(&advisory));
}

#[test]
fn matches_severity() {
    let advisory = load_advisory();

    let query_matches = Query::new().severity(Severity::Critical);
    assert!(query_matches.matches(&advisory));
}

#[test]
fn matches_target_os() {
    let advisory = load_advisory();

    let query_matches = Query::new().target_os(vec!["windows".to_owned(), "linux".to_owned()]);
    assert!(query_matches.matches(&advisory));

    let query_normal = Query::new().target_os(vec!["macos".to_owned(), "freebsd".to_owned()]);
    assert!(!query_normal.matches(&advisory));
}

#[test]
fn matches_target_arch() {
    let advisory = load_advisory();

    let query_matches = Query::new().target_arch(vec!["x86".to_owned(), "arm".to_owned()]);
    assert!(query_matches.matches(&advisory));

    let query_normal = Query::new().target_arch(vec!["mips".to_owned(), "mips64".to_owned()]);
    assert!(!query_normal.matches(&advisory));
}

/// crates.io has two spellings in a lockfile. An advisory that names no source falls back to the
/// `registry+` one, so comparing kind and URL alone dropped every advisory for a package whose
/// lockfile carried the `sparse+` one, even though the package had already been admitted for
/// auditing by the same `is_default_registry` predicate.
#[test]
fn matches_both_crates_io_spellings() {
    let advisory = load_advisory();

    for source in [
        "registry+https://github.com/rust-lang/crates.io-index",
        "sparse+https://index.crates.io/",
    ] {
        let package = package::Package {
            name: "base".parse().unwrap(),
            version: "1.2.2".parse().unwrap(),
            source: Some(source.parse().unwrap()),
            checksum: None,
            dependencies: Default::default(),
            replace: None,
        };

        assert!(
            Query::new().package(&package).matches(&advisory),
            "advisory did not match a package from {source}"
        );
    }
}

/// A genuinely different registry must still not match.
#[test]
fn does_not_match_a_different_registry() {
    let advisory = load_advisory();

    let package = package::Package {
        name: "base".parse().unwrap(),
        version: "1.2.2".parse().unwrap(),
        source: Some(
            "sparse+https://internal.example.com/index/"
                .parse()
                .unwrap(),
        ),
        checksum: None,
        dependencies: Default::default(),
        replace: None,
    };

    assert!(!Query::new().package(&package).matches(&advisory));
}
