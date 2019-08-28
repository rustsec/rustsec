//! Tests for parsing RustSec advisories

/// Example RustSec advisory to use for tests
const ADVISORY_PATH: &str = "./tests/support/example_advisory.toml";

/// Load the advisory from the filesystem
fn load_advisory() -> rustsec::Advisory {
    rustsec::Advisory::load_file(ADVISORY_PATH).unwrap()
}

/// Basic metadata
#[test]
fn parse_metadata() {
    let advisory = load_advisory();
    assert_eq!(advisory.id.as_str(), "RUSTSEC-2001-2101");
    assert_eq!(advisory.package.as_str(), "base");
    assert_eq!(advisory.title, "All your base are belong to us");
    assert_eq!(
        advisory.description,
        "You have no chance to survive. Make your time."
    );
    assert_eq!(advisory.date.as_str(), "2001-02-03");
    assert_eq!(
        advisory.url.unwrap(),
        "https://www.youtube.com/watch?v=jQE66WA2s-A"
    );

    for (i, kw) in ["how", "are", "you", "gentlemen"].iter().enumerate() {
        assert_eq!(*kw, advisory.keywords[i].as_str());
    }
}

/// Parsing of other aliased advisory IDs
#[test]
fn parse_aliases() {
    let alias = &load_advisory().aliases[0];
    assert!(alias.is_cve());
    assert_eq!(alias.year().unwrap(), 2001);
}

/// Parsing of CVSS v3.1 severity vector strings
#[test]
fn parse_cvss_vector_string() {
    let advisory = load_advisory();
    assert_eq!(
        advisory.severity().unwrap(),
        rustsec::advisory::Severity::Critical
    );

    let cvss = advisory.cvss.unwrap();
    assert_eq!(cvss.av.unwrap(), cvss::v3::base::av::AttackVector::Network);
    assert_eq!(cvss.ac.unwrap(), cvss::v3::base::ac::AttackComplexity::Low);
    assert_eq!(
        cvss.pr.unwrap(),
        cvss::v3::base::pr::PrivilegesRequired::None
    );
    assert_eq!(cvss.ui.unwrap(), cvss::v3::base::ui::UserInteraction::None);
    assert_eq!(cvss.s.unwrap(), cvss::v3::base::s::Scope::Changed);
    assert_eq!(cvss.c.unwrap(), cvss::v3::base::c::Confidentiality::High);
    assert_eq!(cvss.i.unwrap(), cvss::v3::base::i::Integrity::High);
    assert_eq!(cvss.a.unwrap(), cvss::v3::base::a::Availability::High);
    assert_eq!(cvss.score().value(), 10.0);
}

/// Parsing of patched version reqs
#[test]
fn parse_patched_version_reqs() {
    let req = &load_advisory().patched_versions[0];
    assert!(!req.matches(&"1.2.2".parse().unwrap()));
    assert!(req.matches(&"1.2.4".parse().unwrap()));
    assert!(req.matches(&"1.2.3".parse().unwrap()));
}
