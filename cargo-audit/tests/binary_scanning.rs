#![cfg(feature = "binary-scanning")]

use std::path::PathBuf;

use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;
use std::{fs, io::Read};
use tempfile::TempDir;

/// Directory containing the advisory database.
///
/// Uses a temporary directory to avoid polluting the default DB.
/// Instead use a single DB we tear down on test suite exit.
static ADVISORY_DB_DIR: Lazy<TempDir> = Lazy::new(|| TempDir::new().unwrap());

/// Executes target binary via `cargo run`.
///
/// Storing this value in a `once_cell::sync::Lazy` ensures that all
/// instances of the runner acquire a mutex when executing commands
/// and inspecting exit statuses, serializing what would otherwise
/// be multithreaded invocations as `cargo test` executes tests in
/// parallel by default.
pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| {
    let mut runner = CmdRunner::default();
    runner
        .arg("audit")
        .arg("--db")
        .arg(ADVISORY_DB_DIR.path())
        .arg("bin");
    runner
});

fn binaries_dir() -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", "support", "binaries"]
        .iter()
        .collect()
}

fn cmd_runner() -> CmdRunner {
    RUNNER.clone()
}

#[test]
fn oversized_binary_is_rejected() {
    let tmpdir = TempDir::new().unwrap();
    let binary_path = tmpdir.path().join("not-a-real-binary");

    // Ensure the file is larger than the configured max size.
    fs::write(&binary_path, b"0123").unwrap();

    let mut runner = cmd_runner();
    runner
        .arg(&binary_path)
        .arg("--max-binary-size")
        .arg("1")
        .capture_stderr();

    let mut process = runner.run();
    let mut stderr = String::new();
    process.stderr().read_to_string(&mut stderr).unwrap();
    process.wait().unwrap().expect_code(2);
    assert!(
        stderr.contains("exceeds max size limit of 1 bytes"),
        "{stderr}"
    );
}

#[test]
fn panicking_binary_without_vulnerabilities_passes() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-without-audit-info");
    assert_eq!(cmd_runner().arg(binary_path).status().code(), 0);
}

#[test]
fn panicking_binary_with_vulnerabilities_fails() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-vuln-panic");
    assert_eq!(cmd_runner().arg(binary_path).status().code(), 1);
}

#[test]
fn auditable_binary_without_vulnerabilities_passes() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-audit-info");
    assert_eq!(cmd_runner().arg(binary_path).status().code(), 0);
}

#[test]
fn auditable_binary_with_vulnerabilities_fails() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-vuln");
    assert_eq!(cmd_runner().arg(binary_path).status().code(), 1);
}
