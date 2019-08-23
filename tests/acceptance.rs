//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.
//!
//! For more information, see:
//! <https://docs.rs/abscissa_core/latest/abscissa_core/testing/index.html>

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::testing::prelude::*;
use lazy_static::lazy_static;
use std::{io::BufRead, path::PathBuf};
use tempfile::TempDir;

lazy_static! {
    /// Directory containing the advisory database.
    ///
    /// Uses a temporary directory to avoid polluting the default DB.
    /// Instead use a single DB we tear down on test suite exit.
    static ref ADVISORY_DB_DIR: TempDir = TempDir::new().unwrap();

    /// Executes target binary via `cargo run`.
    ///
    /// Storing this value in a `lazy_static!` ensures that all instances of
    /// the runner acquire a mutex when executing commands and inspecting
    /// exit statuses, serializing what would otherwise be multithreaded
    /// invocations as `cargo test` executes tests in parallel by default.
    pub static ref RUNNER: CmdRunner = {
        let mut runner = CmdRunner::default();
        runner.arg("audit").arg("--db").arg(ADVISORY_DB_DIR.path());
        runner.capture_stdout().capture_stderr();
        runner
    };
}

/// Get a `CmdRunner` configured to point at a project with or without vulns
pub fn new_cmd_runner(has_vulns: bool) -> CmdRunner {
    let mut runner = RUNNER.clone();
    let project = if has_vulns { "base64_vuln" } else { "no_vulns" };

    let tests_data_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "support"]
        .iter()
        .collect();

    // Point at the integration test example project's Cargo.lock file.
    runner
        .arg("--file")
        .arg(tests_data_dir.join(project).join("Cargo.lock"));

    runner
}

/// Get the advisory JSON output from a `CmdRunner`
pub fn get_advisories_json(process: &mut Process) -> serde_json::Value {
    let mut output = String::new();
    process.stdout().read_line(&mut output).unwrap();
    serde_json::from_str(&output).unwrap()
}

#[test]
fn no_advisories_found_exit_success() {
    new_cmd_runner(false).status().expect_success();
}

#[test]
fn advisories_found_exit_error() {
    new_cmd_runner(true).status().expect_code(1);
}

#[test]
fn no_advisories_found_empty_json() {
    let mut runner = new_cmd_runner(false);
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
    let mut runner = new_cmd_runner(true);
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
    let mut runner = new_cmd_runner(true);
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
