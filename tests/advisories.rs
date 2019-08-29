//! Tests for parsing RustSec advisories

use rustsec::advisory::Category;

/// Example RustSec Advisory (V1 format) to use for tests
const ADVISORY_PATH_V1: &str = "./tests/support/example_advisory_v1.toml";

/// Example RustSec Advisory (V2 format) to use for tests
const ADVISORY_PATH_V2: &str = "./tests/support/example_advisory_v2.toml";

/// Load the V1 advisory from the filesystem
fn load_advisory_v1() -> rustsec::Advisory {
    rustsec::Advisory::load_file(ADVISORY_PATH_V1).unwrap()
}

/// Load the V1 advisory from the filesystem
fn load_advisory_v2() -> rustsec::Advisory {
    rustsec::Advisory::load_file(ADVISORY_PATH_V2).unwrap()
}

/// Basic metadata
#[test]
fn parse_metadata() {
    let advisory = load_advisory_v1();
    assert_eq!(advisory.info.id.as_str(), "RUSTSEC-2001-2101");
    assert_eq!(advisory.info.package.as_str(), "base");
    assert_eq!(advisory.info.title, "All your base are belong to us");
    assert_eq!(
        advisory.info.description,
        "You have no chance to survive. Make your time."
    );
    assert_eq!(advisory.info.date.as_str(), "2001-02-03");
    assert_eq!(
        advisory.info.url.unwrap(),
        "https://www.youtube.com/watch?v=jQE66WA2s-A"
    );

    for (i, category) in [Category::PrivilegeEscalation, Category::RemoteCodeExecution]
        .iter()
        .enumerate()
    {
        assert_eq!(*category, advisory.info.categories[i]);
    }

    for (i, kw) in ["how", "are", "you", "gentlemen"].iter().enumerate() {
        assert_eq!(*kw, advisory.info.keywords[i].as_str());
    }
}

/// Parsing of impact metadata
#[test]
fn parse_affected() {
    let affected = load_advisory_v1().affected.unwrap();
    assert_eq!(affected.arch[0], platforms::target::Arch::X86);
    assert_eq!(affected.os[0], platforms::target::OS::Windows);

    let example_function = "base::belongs::All".parse().unwrap();
    let req = &affected.functions.get(&example_function).unwrap()[0];
    assert!(req.matches(&"1.2.2".parse().unwrap()));
    assert!(!req.matches(&"1.2.3".parse().unwrap()));
}

/// Parsing of other aliased advisory IDs
#[test]
fn parse_aliases() {
    let alias = &load_advisory_v1().info.aliases[0];
    assert!(alias.is_cve());
    assert_eq!(alias.year().unwrap(), 2001);
}

/// Parsing of CVSS v3.1 severity vector strings
#[test]
fn parse_cvss_vector_string() {
    let advisory = load_advisory_v1();
    assert_eq!(
        advisory.severity().unwrap(),
        rustsec::advisory::Severity::Critical
    );

    let cvss = advisory.info.cvss.unwrap();
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

/// Parsing of patched version reqs (V1 format)
#[test]
fn parse_patched_version_reqs_v1() {
    let req = &load_advisory_v1().versions.patched[0];
    assert!(!req.matches(&"1.2.2".parse().unwrap()));
    assert!(req.matches(&"1.2.3".parse().unwrap()));
    assert!(req.matches(&"1.2.4".parse().unwrap()));
}

/// Parsing of patched version reqs (V2 format)
#[test]
fn parse_patched_version_reqs_v2() {
    let req = &load_advisory_v2().versions.patched[0];
    assert!(!req.matches(&"1.2.2".parse().unwrap()));
    assert!(req.matches(&"1.2.3".parse().unwrap()));
    assert!(req.matches(&"1.2.4".parse().unwrap()));
}

/// Ensure V1 and V2 formats parse equivalently
#[test]
fn advisory_v1_and_v2_equivalence() {
    assert_eq!(load_advisory_v1(), load_advisory_v2());
}
