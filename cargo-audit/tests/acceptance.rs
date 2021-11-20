//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.
//!
//! For more information, see:
//! <https://docs.rs/abscissa_core/latest/abscissa_core/testing/index.html>

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;
use std::{io::BufRead, path::PathBuf};
use std::fs::File;
use tempfile::TempDir;
use rustsec::Report;

/// Directory containing the advisory database.
///
/// Uses a temporary directory to avoid polluting the default DB.
/// Instead use a single DB we tear down on test suite exit.
static ADVISORY_DB_DIR: Lazy<TempDir> = Lazy::new(|| TempDir::new().unwrap());

/// Executes target binary via `cargo run`.
///
/// Storing this value in a `lazy_static!` ensures that all instances of
/// the runner acquire a mutex when executing commands and inspecting
/// exit statuses, serializing what would otherwise be multithreaded
/// invocations as `cargo test` executes tests in parallel by default.
pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| {
    let mut runner = CmdRunner::default();
    runner.arg("audit").arg("--db").arg(ADVISORY_DB_DIR.path());
    runner.capture_stdout().capture_stderr();
    runner
});

/// Get a `CmdRunner` configured to point at a project with or without vulns
fn new_cmd_runner(project: &str) -> CmdRunner {
    let mut runner = RUNNER.clone();

    let tests_data_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "support"]
        .iter()
        .collect();

    // Point at the integration test example project's Cargo.lock file.
    runner
        .arg("--file")
        .arg(tests_data_dir.join(project).join("Cargo.lock"));

    runner
}

/// Get a `CmdRunner` to a project which contains vulnerabilities.
pub fn vulnerable_cmd_runner() -> CmdRunner {
    new_cmd_runner("base64_vuln")
}

/// Get a `CmdRunner` to a project without vulnerabilities.
pub fn secure_cmd_runner() -> CmdRunner {
    new_cmd_runner("no_vulns")
}

/// Get a `CmdRunner` to a project without any Cargo.toml or Cargo.lock.
pub fn failing_cmd_runner() -> CmdRunner {
    new_cmd_runner("empty")
}

/// Get the advisory JSON output from a `CmdRunner`
pub fn get_advisories_json(process: &mut Process) -> serde_json::Value {
    let mut output = String::new();
    process.stdout().read_line(&mut output).unwrap();
    serde_json::from_str(&output).unwrap()
}

#[test]
fn no_advisories_found_exit_success() {
    secure_cmd_runner().status().expect_success();
}

#[test]
fn advisories_found_exit_error() {
    vulnerable_cmd_runner().status().expect_code(1);
}

#[test]
fn no_lockfile_exit_error() {
    failing_cmd_runner().status().expect_code(2);
}

#[test]
fn no_advisories_found_empty_json() {
    let mut runner = secure_cmd_runner();
    runner.arg("--json");

    let mut process = runner.run();
    let json = get_advisories_json(&mut process);
    process.wait().unwrap().expect_success();

    assert_eq!(
        json.pointer("/vulnerabilities/count")
            .unwrap()
            .as_u64()
            .unwrap(),
        0
    );

    let vulnerabilities = json
        .pointer("/vulnerabilities/list")
        .unwrap()
        .as_array()
        .unwrap();

    assert!(vulnerabilities.is_empty())
}

#[test]
fn advisories_found_json() {
    let mut runner = vulnerable_cmd_runner();
    runner.arg("--json");

    let mut process = runner.run();
    let json = get_advisories_json(&mut process);
    process.wait().unwrap().expect_code(1);

    assert_eq!(
        json.pointer("/vulnerabilities/count")
            .unwrap()
            .as_u64()
            .unwrap(),
        1
    );

    let vulnerabilities = json
        .pointer("/vulnerabilities/list")
        .unwrap()
        .as_array()
        .unwrap();

    assert_eq!(vulnerabilities.len(), 1);

    let advisory_id = vulnerabilities[0]
        .pointer("/advisory/id")
        .unwrap()
        .as_str()
        .unwrap();

    assert_eq!(advisory_id, "RUSTSEC-2017-0004");
}

#[test]
fn version() {
    let mut runner = RUNNER.clone();
    runner.arg("--version");
    let process = runner.run();
    process.wait().unwrap().expect_success();
}

#[test]
fn advisories_found_but_ignored_json() {
    let mut runner = vulnerable_cmd_runner();
    runner.arg("--json");
    runner.arg("--ignore").arg("RUSTSEC-2017-0004");

    let mut process = runner.run();
    let json = get_advisories_json(&mut process);
    process.wait().unwrap().expect_success();

    assert_eq!(
        json.pointer("/vulnerabilities/count")
            .unwrap()
            .as_u64()
            .unwrap(),
        0
    );
}

#[test]
fn parse_affected_from_json() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "support", "json", "audit.json"]
        .iter()
        .collect();
    let file = File::open(path)
        .expect("cannot open file");
    let report = serde_json::from_reader::<_, Report>(file);
    assert!(report.is_ok())
}
