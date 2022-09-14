#![cfg(feature = "binary-scanning")]

use std::path::PathBuf;

use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;
use tempfile::TempDir;

/// Directory containing the advisory database.
///
/// Uses a temporary directory to avoid polluting the default DB.
/// Instead use a single DB we tear down on test suite exit.
static ADVISORY_DB_DIR: Lazy<TempDir> = Lazy::new(|| TempDir::new().unwrap());

fn binaries_dir() -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", "support", "binaries"]
        .iter()
        .collect()
}

fn cmd_runner() -> CmdRunner {
    let mut runner = CmdRunner::default();
    runner
        .arg("audit")
        .arg("bin")
        .arg("--db")
        .arg(ADVISORY_DB_DIR.path());
    runner
}

#[test]
fn binary_without_audit_info_is_rejected() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-without-audit-info");
    assert_eq!(cmd_runner().arg(binary_path).status().code(), 2);
}

#[test]
fn binary_without_vulnerabilities_passes() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-audit-info");
    assert_eq!(cmd_runner().arg(binary_path).status().code(), 0);
}

#[test]
fn binary_with_vulnerabilities_fails() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-vuln");
    assert_eq!(cmd_runner().arg(binary_path).status().code(), 1);
}
