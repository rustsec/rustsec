#![cfg(feature = "binary-scanning")]

use std::path::PathBuf;

use once_cell::sync::Lazy;
use std::{
    fs,
    process::{Command, Output},
    sync::{Mutex, PoisonError},
};
use tempfile::TempDir;

/// Directory containing the advisory database.
///
/// Uses a temporary directory to avoid polluting the default DB.
/// Instead use a single DB we tear down on test suite exit.
static ADVISORY_DB_DIR: Lazy<TempDir> = Lazy::new(|| TempDir::new().unwrap());

/// Serializes invocations of the target binary, which would otherwise be
/// multithreaded as `cargo test` executes tests in parallel by default.
static LOCK: Mutex<()> = Mutex::new(());

/// Run the given command to completion, capturing its output
fn run(cmd: &mut Command) -> Output {
    let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    cmd.output().unwrap()
}

fn binaries_dir() -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", "support", "binaries"]
        .iter()
        .collect()
}

/// Get a `Command` which executes `cargo audit bin` with the target binary
fn cmd_runner() -> Command {
    let mut runner = Command::new(env!("CARGO_BIN_EXE_cargo-audit"));
    runner
        .arg("audit")
        .arg("--db")
        .arg(ADVISORY_DB_DIR.path())
        .arg("bin");
    runner
}

#[test]
fn oversized_binary_is_rejected() {
    let tmpdir = TempDir::new().unwrap();
    let binary_path = tmpdir.path().join("not-a-real-binary");

    // Ensure the file is larger than the configured max size.
    fs::write(&binary_path, b"0123").unwrap();

    let mut runner = cmd_runner();
    runner.arg(&binary_path).arg("--max-binary-size").arg("1");

    let output = run(&mut runner);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2));
    assert!(
        stderr.contains("exceeds max size limit of 1 bytes"),
        "{stderr}"
    );
}

#[test]
fn panicking_binary_without_vulnerabilities_passes() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-without-audit-info");
    assert_eq!(run(cmd_runner().arg(binary_path)).status.code(), Some(0));
}

#[test]
fn panicking_binary_with_vulnerabilities_fails() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-vuln-panic");
    assert_eq!(run(cmd_runner().arg(binary_path)).status.code(), Some(1));
}

#[test]
fn auditable_binary_without_vulnerabilities_passes() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-audit-info");
    assert_eq!(run(cmd_runner().arg(binary_path)).status.code(), Some(0));
}

#[test]
fn auditable_binary_with_vulnerabilities_fails() {
    let mut binary_path = binaries_dir();
    binary_path.push("binary-with-vuln");
    assert_eq!(run(cmd_runner().arg(binary_path)).status.code(), Some(1));
}
