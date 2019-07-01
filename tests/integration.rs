extern crate assert_cmd;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate tempfile;

use assert_cmd::cargo::cargo_bin;
use assert_cmd::prelude::*;
use serial_test_derive::serial;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

lazy_static! {
    static ref DB_DIR: TempDir = TempDir::new().unwrap();
}

fn cargo_audit(project: &str) -> Command {
    // The cargo-audit binary expects to be called in a cargo subcommand context; eg: `cargo audit`.
    let mut command = Command::new(env!("CARGO"));
    command
        .arg("audit")
        .env("PATH", cargo_bin("cargo-audit").parent().unwrap());

    // Don't pollute the default DB; instead use a single DB we tear down on test suite exit.
    command.arg("--db").arg(DB_DIR.path());

    let tests_data_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "support"]
        .iter()
        .collect();

    // Point at the integration test example project's Cargo.lock file.
    command
        .arg("--file")
        .arg(tests_data_dir.join(project).join("Cargo.lock"));

    command
}

fn assert_no_advisories(command: &mut Command) {
    let output = command.arg("--json").unwrap();
    assert_advisory_output(&output, vec![]);
}

fn assert_advisories(command: &mut Command, expected_advisories: Vec<&str>) {
    let output = command.arg("--json").unwrap_err();
    assert_advisory_output(output.as_output().unwrap(), expected_advisories);
}

fn assert_advisory_output(output: &std::process::Output, expected_advisories: Vec<&str>) {
    let json: serde_json::Value =
        serde_json::from_slice(output.stdout.as_slice()).unwrap();

    // Example JSON structure:
    //
    //{
    //  "database": {
    //    "advisory-count": 24
    //  },
    //  "lockfile": {
    //    "dependency-count": 318,
    //    "path": "..."
    //  },
    //  "vulnerabilities": {
    //    "count": 1,
    //    "found": true,
    //    "list": [
    //      {
    //        "advisory": {
    //          "affected_arch": null,
    //          "affected_os": null,
    //          "affected_paths": null,
    //          "aliases": [],
    //          "date": "2018-06-08",
    //          "description": "...",
    //          "id": "RUSTSEC-2019-0003",
    //          "keywords": [
    //            "oom",
    //            "panic",
    //            "dos"
    //          ],
    //          "package": "protobuf",
    //          "patched_versions": [],
    //          "references": [],
    //          "title": "Out of Memory in stream::read_raw_bytes_into()",
    //          "unaffected_versions": [],
    //          "url": "https://github.com/stepancheg/rust-protobuf/issues/411"
    //        },
    //        "package": {
    //          "dependencies": [
    //            "bytes 0.4.12 (registry+https://github.com/rust-lang/crates.io-index)"
    //          ],
    //          "name": "protobuf",
    //          "source": "registry+https://github.com/rust-lang/crates.io-index",
    //          "version": "2.0.6"
    //        }
    //      }
    //    ]
    //  }
    //}

    let expected: HashSet<&str> = expected_advisories.into_iter().collect();
    assert_eq!(
        expected.len() as u64,
        json.pointer("/vulnerabilities/count")
            .unwrap()
            .as_u64()
            .unwrap()
    );
    assert_eq!(
        expected.len() > 0,
        json.pointer("/vulnerabilities/found")
            .unwrap()
            .as_bool()
            .unwrap()
    );

    let vulnerabilities = json
        .pointer("/vulnerabilities/list")
        .unwrap()
        .as_array()
        .unwrap();

    let actual_advisories: HashSet<&str> = vulnerabilities
        .into_iter()
        .map(|value| value.pointer("/advisory/id").unwrap().as_str().unwrap())
        .collect();
    assert_eq!(expected, actual_advisories);
}

#[test]
#[serial]
fn no_vulnerabilities_json_valid() {
    assert_no_advisories(&mut cargo_audit("no_vulns"));
}

#[test]
#[serial]
fn ignore() {
    assert_advisories(&mut cargo_audit("base64_vuln"), vec!["RUSTSEC-2017-0004"]);

    let mut ignore_typo_command = cargo_audit("base64_vuln");
    ignore_typo_command.arg("--ignore").arg("RUSTSEC-2017-0003");
    assert_advisories(&mut ignore_typo_command, vec!["RUSTSEC-2017-0004"]);

    assert_no_advisories(&mut (cargo_audit("no_vulns")
        .arg("--ignore")
        .arg("RUSTSEC-2017-0004")));
}
