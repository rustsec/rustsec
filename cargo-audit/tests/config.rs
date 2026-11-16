//! Configuration file tests

use std::{fs, path::Path};

use cargo_audit::config::{AuditConfig, OutputFormat};

#[test]
fn default_output_config() {
    let output = AuditConfig::default().output;
    assert!(output.show_tree);
    assert!(output.deny.is_empty());
    assert_eq!(output.format, OutputFormat::Terminal);
    assert!(!output.quiet);
}

#[test]
fn configured_dependency_trees() {
    for (input, show_tree) in [
        ("", true),
        ("[target]\narch = 'x86_64'", true),
        ("[output]\nquiet = false", true),
        ("[output]\nquiet = false\nshow_tree = true", true),
        ("[output]\nquiet = false\nshow_tree = false", false),
    ] {
        let config: AuditConfig = toml::from_str(input).unwrap();
        assert_eq!(config.output.show_tree, show_tree, "{input}");
    }
}

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
    assert_eq!(config.target.arch(), vec!["x86_64".to_owned()]);
    assert_eq!(
        config.target.os(),
        vec!["linux".to_owned(), "windows".to_owned()]
    );
}

/// Ensure `target.arch` and `target.os` continue to parse when they
/// are specified as a string, and not a list. This is the legacy behavior.
#[test]
fn parser_audit_toml_example() {
    let toml_string = fs::read_to_string("tests/audit.toml.legacy").unwrap();
    let config: AuditConfig = toml::from_str(&toml_string).unwrap();

    assert_eq!(config.target.arch(), vec!["x86_64".to_owned()]);
    assert_eq!(config.target.os(), vec!["linux".to_owned()]);
}
