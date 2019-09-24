//! Configuration file tests

use cargo_audit::config::AuditConfig;
use std::{fs, path::Path};

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
}
