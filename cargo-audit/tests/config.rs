//! Configuration file tests

use std::{fs, path::Path};

use cargo_audit::config::AuditConfig;
use rustsec::platforms::{Arch, OS};

/// Ensure `audit.toml.example` parses as a valid config file
#[test]
fn parse_audit_toml_example() {
    let toml_string = fs::read_to_string("audit.toml.example").unwrap();
    let config: AuditConfig = toml::from_str(&toml_string).unwrap();

    assert_eq!(
        config.database.path.unwrap(),
        Path::new("~/.cargo/advisory-db")
    );
    assert_eq!(
        config.database.url.unwrap(),
        "https://github.com/RustSec/advisory-db.git"
    );
    assert_eq!(config.target.arch(), vec![Arch::X86_64]);
    assert_eq!(config.target.os(), vec![OS::Linux, OS::Windows]);
}

/// Ensure `target.arch` and `target.os` continue to parse when they
/// are specified as a string, and not a list. This is the legacy behovior.
#[test]
fn parser_audit_toml_example() {
    let toml_string = fs::read_to_string("tests/audit.toml.legacy").unwrap();
    let config: AuditConfig = toml::from_str(&toml_string).unwrap();

    assert_eq!(config.target.arch(), vec![Arch::X86_64]);
    assert_eq!(config.target.os(), vec![OS::Linux]);
}
