//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.

#![forbid(unsafe_code)]

use once_cell::sync::Lazy;
use std::{
    collections::HashSet,
    path::PathBuf,
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

/// Get a `Command` which executes the target binary
pub fn cmd_runner() -> Command {
    let mut runner = Command::new(env!("CARGO_BIN_EXE_cargo-audit"));
    runner.arg("audit").arg("--db").arg(ADVISORY_DB_DIR.path());
    runner
}

/// Run the given command to completion, capturing its output
pub fn run(cmd: &mut Command) -> Output {
    let _guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    cmd.output().unwrap()
}

/// Get a `Command` configured to point at a project with or without vulns
fn new_cmd_runner(project: &str) -> Command {
    let mut runner = cmd_runner();

    let tests_data_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "support"]
        .iter()
        .collect();

    // Point at the integration test example project's Cargo.lock file.
    runner
        .arg("--file")
        .arg(tests_data_dir.join(project).join("Cargo.lock"));

    runner
}

/// Get a `Command` to a project which contains vulnerabilities.
pub fn vulnerable_cmd_runner() -> Command {
    new_cmd_runner("base64_vuln")
}

/// Get a `Command` to a project without vulnerabilities.
pub fn secure_cmd_runner() -> Command {
    new_cmd_runner("no_vulns")
}

/// Get a `Command` to a project without any Cargo.toml or Cargo.lock.
pub fn failing_cmd_runner() -> Command {
    new_cmd_runner("empty")
}

/// Get a `Command` to a project with an unmaintained dependency
pub fn unmaintained_cmd_runner() -> Command {
    new_cmd_runner("unmaintained")
}

/// Get a `Command` to a project with a yanked dependency
pub fn yanked_cmd_runner() -> Command {
    new_cmd_runner("yanked")
}

/// Get a `Command` to a project with an unsound dependency
pub fn unsound_cmd_runner() -> Command {
    new_cmd_runner("unsound")
}

/// Get a `Command` to a project with a dependency with a notice advisory
pub fn notice_cmd_runner() -> Command {
    new_cmd_runner("notice")
}

/// Get a `Command` to a project with vulnerable dependencies that have
/// unaffected newer versions available to upgrade to.
pub fn unaffected_newer_cmd_runner() -> Command {
    new_cmd_runner("unaffected-newer")
}

/// Get the advisory JSON output from a command's output
pub fn get_advisories_json(output: &Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().unwrap_or_default();
    dbg!(&line);
    serde_json::from_str(line).unwrap()
}

#[test]
fn no_advisories_found_exit_success() {
    assert_eq!(run(&mut secure_cmd_runner()).status.code(), Some(0));
}

#[test]
fn advisories_found_exit_error() {
    assert_eq!(run(&mut vulnerable_cmd_runner()).status.code(), Some(1));
}

#[test]
fn no_lockfile_exit_error() {
    assert_eq!(run(&mut failing_cmd_runner()).status.code(), Some(2));
}

#[test]
fn unmaintained_exit_success_by_default() {
    assert_eq!(run(&mut unmaintained_cmd_runner()).status.code(), Some(0));
}

#[test]
fn unmaintained_exit_failure_deny_warnings() {
    let mut runner = unmaintained_cmd_runner();
    runner.arg("--deny=warnings");
    let output = run(&mut runner);
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn yanked_exit_success_by_default() {
    let mut runner = yanked_cmd_runner();
    let output = run(&mut runner);
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn yanked_exit_failure_deny_yanked() {
    let mut runner = yanked_cmd_runner();
    runner.arg("--deny=yanked");
    let output = run(&mut runner);
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn no_advisories_found_empty_json() {
    let mut runner = secure_cmd_runner();
    runner.arg("--json");

    let output = run(&mut runner);
    let json = get_advisories_json(&output);
    assert_eq!(output.status.code(), Some(0));

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

    let output = run(&mut runner);
    let json = get_advisories_json(&output);
    assert_eq!(output.status.code(), Some(1));

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
    let mut runner = cmd_runner();
    runner.arg("--version");
    let output = run(&mut runner);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("cargo-audit-audit {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn advisories_found_but_ignored_json() {
    let mut runner = vulnerable_cmd_runner();
    runner.arg("--json");
    runner.arg("--ignore").arg("RUSTSEC-2017-0004");

    let output = run(&mut runner);
    let json = get_advisories_json(&output);
    assert_eq!(output.status.code(), Some(0));

    assert_eq!(
        json.pointer("/vulnerabilities/count")
            .unwrap()
            .as_u64()
            .unwrap(),
        0
    );
}

#[test]
fn unmaintained_advisories_found_json() {
    let mut runner = unmaintained_cmd_runner();
    runner.arg("--json");

    let output = run(&mut runner);
    let json = get_advisories_json(&output);
    assert_eq!(output.status.code(), Some(0));

    let unmaintained_warnings = json
        .pointer("/warnings/unmaintained")
        .unwrap()
        .as_array()
        .unwrap();

    assert_eq!(unmaintained_warnings.len(), 1);

    let advisory_id = unmaintained_warnings[0]
        .pointer("/advisory/id")
        .unwrap()
        .as_str()
        .unwrap();

    assert_eq!(advisory_id, "RUSTSEC-2022-0061");
}

#[test]
fn denied_warnings_json() {
    for (deny, denied) in [
        (None, false),
        (Some("unmaintained"), true),
        (Some("unsound"), false),
    ] {
        let mut runner = unmaintained_cmd_runner();
        runner.arg("--json").arg("--no-yanked");
        if let Some(deny) = deny {
            runner.arg(format!("--deny={deny}"));
        }
        let output = run(&mut runner);
        let json = get_advisories_json(&output);
        let warnings = json["warnings"]["unmaintained"].as_array().unwrap();
        assert!(!warnings.is_empty());
        for warning in warnings {
            assert_eq!(
                warning.get("denied").and_then(|value| value.as_bool()),
                denied.then_some(true)
            );
        }
        assert_eq!(output.status.code(), Some(i32::from(denied)));
    }
}

#[test]
fn unsound_advisories_found_json() {
    let mut runner = unsound_cmd_runner();
    runner.arg("--json");

    let output = run(&mut runner);
    let json = get_advisories_json(&output);
    assert_eq!(output.status.code(), Some(0));

    let unsound_warnings = json
        .pointer("/warnings/unsound")
        .unwrap()
        .as_array()
        .unwrap();

    assert_eq!(unsound_warnings.len(), 1);

    let advisory_id = unsound_warnings[0]
        .pointer("/advisory/id")
        .unwrap()
        .as_str()
        .unwrap();

    assert_eq!(advisory_id, "RUSTSEC-2021-0090");
}

#[test]
fn notice_advisories_found_json() {
    let mut runner = notice_cmd_runner();
    runner.arg("--json");

    let output = run(&mut runner);
    let json = get_advisories_json(&output);
    assert_eq!(output.status.code(), Some(0));

    let notice_warnings = json
        .pointer("/warnings/notice")
        .unwrap()
        .as_array()
        .unwrap();

    assert_eq!(notice_warnings.len(), 1);

    let advisory_id = notice_warnings[0]
        .pointer("/advisory/id")
        .unwrap()
        .as_str()
        .unwrap();

    assert_eq!(advisory_id, "RUSTSEC-2022-0058");
}

#[test]
fn unaffected_newer_suggests_upgrades() {
    let mut runner = unaffected_newer_cmd_runner();
    runner.arg("--json");

    let output = run(&mut runner);
    let json = get_advisories_json(&output);
    assert_eq!(output.status.code(), Some(1));

    // There should be (at least) two advisories:
    //
    // 1. RUSTSEC-2026-0118, which will suggest an unaffected version >= 0.26.0-beta.1,
    // 2. RUSTSEC-2026-0119, which will suggest a patched version >= 0.26.1.
    //
    // We're going to verify that these are in the JSON output before we re-run
    // the audit to validate the presenter output. If they aren't, then this
    // test may need to be regenerated against the current advisory database.
    let vulns = json
        .pointer("/vulnerabilities/list")
        .unwrap()
        .as_array()
        .unwrap();

    let ids = HashSet::<&str>::from_iter(
        vulns
            .iter()
            .map(|v| v.pointer("/advisory/id").unwrap().as_str().unwrap()),
    );

    assert!(ids.contains("RUSTSEC-2026-0118"));
    assert!(ids.contains("RUSTSEC-2026-0119"));

    // OK, now let's re-run without the --json flag and verify that the
    // presenter rendered the suggested solutions as expected.
    let output = run(&mut unaffected_newer_cmd_runner());

    // We'll just grab the Solution: lines, since that's all we're interested in.
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout
        .lines()
        .filter(|line| line.starts_with("Solution:"))
        .collect::<Vec<_>>();
    assert_eq!(output.status.code(), Some(1));

    // These expectations are based on the same advisories as listed above. Note
    // that we're interested both in whether the solutions exist, and that they
    // are correctly annotated as being based on the version being unaffected or
    // patched, respectively.
    for expected in [
        "Upgrade to >=0.26.0-beta.1 (unaffected)",
        "Upgrade to >=0.26.1 (patched)",
    ] {
        assert!(
            lines.iter().any(|line| line.contains(expected)),
            "looking for: {expected}"
        );
    }
}

// Causes tests to time out when run from tests, but works when invoked normally
// TODO: re-enable
// #[test]
// fn branch_regression() {
//     // Verifies that we don't crash on this Cargo.lock
//     assert_eq!(run(&mut new_cmd_runner("branch-regression")).status.code(), Some(1));
// }
