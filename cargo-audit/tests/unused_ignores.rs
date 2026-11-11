//! Offline CLI tests for identifying ignore entries no longer needed by a lockfile.

use std::{
    fs,
    process::{Command, Output},
};
use tempfile::TempDir;

const ID: &str = "RUSTSEC-2024-0001";
const UNKNOWN: &str = "RUSTSEC-2024-9999";
const WARNING: &str = "is not needed for this lockfile";

struct Fixture(TempDir);

impl Fixture {
    fn new(version: &str, source: Option<&str>, metadata: &str) -> Self {
        let dir = TempDir::new().unwrap();
        let path = dir.path();
        fs::create_dir_all(path.join("db/crates/example")).unwrap();
        fs::write(path.join("db/crates/example/RUSTSEC-2024-0001.md"), format!(
            "```toml\n[advisory]\nid = \"{ID}\"\npackage = \"example\"\ndate = \"2024-01-01\"\n{metadata}\n[versions]\npatched = [\">= 0.2.0\"]\n```\n\n# Test advisory\n\nOffline fixture.\n"
        )).unwrap();
        let source = source
            .map(|s| format!("source = \"{s}\"\n"))
            .unwrap_or_default();
        fs::write(
            path.join("Cargo.lock"),
            format!(
                "version = 3\n\n[[package]]\nname = \"example\"\nversion = \"{version}\"\n{source}"
            ),
        )
        .unwrap();
        fs::create_dir(path.join(".cargo")).unwrap();
        fs::write(path.join(".cargo/audit.toml"), "").unwrap();
        Self(dir)
    }

    fn registry(version: &str, metadata: &str) -> Self {
        Self::new(
            version,
            Some("registry+https://github.com/rust-lang/crates.io-index"),
            metadata,
        )
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_cargo-audit"))
            .current_dir(self.0.path())
            .args([
                "audit",
                "--db",
                "db",
                "--no-fetch",
                "--no-yanked",
                "--color",
                "never",
            ])
            .args(args)
            .output()
            .unwrap()
    }

    fn config(&self, text: &str) {
        fs::write(self.0.path().join(".cargo/audit.toml"), text).unwrap();
    }
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).unwrap()
}

#[test]
fn unused_ignore_warnings_are_on_by_default_and_can_be_disabled() {
    let fixture = Fixture::registry("0.2.0", "");

    let output = fixture.run(&["--ignore", ID]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stderr(&output).contains(&format!("{ID} {WARNING}")));

    let output = fixture.run(&["--ignore", ID, "--warn-unused-ignores=false"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(!stderr(&output).contains(WARNING));

    fixture.config("[advisories]\nwarn_unused_ignores = false\n");
    let output = fixture.run(&["--ignore", ID]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(!stderr(&output).contains(WARNING));
}

#[test]
fn cli_flag_overrides_configuration_in_both_directions() {
    let fixture = Fixture::registry("0.2.0", "");
    fixture.config("[advisories]\nwarn_unused_ignores = false\n");

    let output = fixture.run(&["--ignore", ID, "--warn-unused-ignores"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stderr(&output).contains(&format!("{ID} {WARNING}")));

    let output = fixture.run(&["--ignore", ID, "--warn-unused-ignores=true"]);
    assert!(stderr(&output).contains(&format!("{ID} {WARNING}")));

    fixture.config("[advisories]\nwarn_unused_ignores = true\n");
    let output = fixture.run(&["--ignore", ID, "--warn-unused-ignores=false"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(!stderr(&output).contains(WARNING));
}

#[test]
fn warns_for_patched_versions_without_changing_exit_status() {
    let fixture = Fixture::registry("0.2.0", "");
    let output = fixture.run(&["--ignore", ID, "--deny", "warnings"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stderr(&output).contains(&format!("{ID} {WARNING}")));
}

#[test]
fn affected_ignores_still_suppress_vulnerabilities() {
    let fixture = Fixture::registry("0.1.0", "");
    let output = fixture.run(&["--ignore", ID]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(!stderr(&output).contains(WARNING));
    assert_eq!(fixture.run(&[]).status.code(), Some(1));
}

#[test]
fn informational_and_filtered_advisories_still_need_ignores() {
    for metadata in [
        "informational = \"unmaintained\"",
        "cvss = \"CVSS:3.1/AV:L/AC:H/PR:H/UI:R/S:U/C:L/I:N/A:N\"",
        "\n[affected]\nos = [\"windows\"]\narch = [\"x86\"]",
    ] {
        let fixture = Fixture::registry("0.1.0", metadata);
        fixture.config(
            "[advisories]\ninformational_warnings = []\nseverity_threshold = \"critical\"\n",
        );
        let output = fixture.run(&[
            "--ignore",
            ID,
            "--target-os",
            "linux",
            "--target-arch",
            "aarch64",
        ]);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(!stderr(&output).contains(WARNING), "{}", stderr(&output));
    }
}

#[test]
fn config_and_cli_ignores_are_both_checked() {
    let fixture = Fixture::registry("0.2.0", "");
    fixture.config(&format!(
        "[advisories]\nignore = [\"{ID}\"]\nwarn_unused_ignores = true\n"
    ));
    let output = fixture.run(&["--ignore", UNKNOWN]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stderr(&output).matches(WARNING).count(), 2);
    assert!(stderr(&output).contains(&format!("{ID} {WARNING}")));
    assert!(stderr(&output).contains(&format!("{UNKNOWN} {WARNING}")));
}

#[test]
fn withdrawn_and_different_source_ignores_are_unused() {
    for fixture in [
        Fixture::registry("0.1.0", "withdrawn = \"2024-02-01\""),
        Fixture::new("0.1.0", None, ""),
        Fixture::new(
            "0.1.0",
            Some("git+https://example.com/repo#0123456789012345678901234567890123456789"),
            "",
        ),
        Fixture::new("0.1.0", Some("registry+https://example.com/index"), ""),
    ] {
        let output = fixture.run(&["--ignore", ID]);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(
            stderr(&output).contains(&format!("{ID} {WARNING}")),
            "{}",
            stderr(&output)
        );
    }
}

#[test]
fn warnings_do_not_change_json_or_vulnerability_exit_status() {
    let fixture = Fixture::registry("0.1.0", "");
    let output = fixture.run(&["--ignore", UNKNOWN, "--json"]);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["vulnerabilities"]["count"], 1);
    assert!(stderr(&output).contains(&format!("{UNKNOWN} {WARNING}")));
}

#[test]
fn absent_packages_are_unused_but_any_affected_version_keeps_ignore() {
    let fixture = Fixture::registry("0.2.0", "");
    fs::write(fixture.0.path().join("Cargo.lock"), "version = 3\n").unwrap();
    let output = fixture.run(&["--ignore", ID]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stderr(&output).contains(WARNING));
    fs::write(fixture.0.path().join("Cargo.lock"), "version = 3\n\n[[package]]\nname = \"example\"\nversion = \"0.1.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n\n[[package]]\nname = \"example\"\nversion = \"0.2.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n").unwrap();
    let output = fixture.run(&["--ignore", ID]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(!stderr(&output).contains(WARNING));
}

#[test]
fn warnings_are_visible_in_quiet_mode() {
    let fixture = Fixture::registry("0.2.0", "");
    let output = fixture.run(&["--ignore", ID, "--quiet"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stdout.is_empty());
    assert!(stderr(&output).contains(WARNING));
}

#[test]
fn warnings_do_not_change_sarif_output() {
    let fixture = Fixture::registry("0.2.0", "");
    let output = fixture.run(&["--ignore", ID, "--format", "sarif"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["version"], "2.1.0");
    assert!(stderr(&output).contains(WARNING));
}

#[test]
fn warnings_without_ignores_leave_report_unchanged() {
    let fixture = Fixture::registry("0.1.0", "");
    let output = fixture.run(&["--json"]);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["vulnerabilities"]["count"], 1);
    assert!(!stderr(&output).contains(WARNING));
}
