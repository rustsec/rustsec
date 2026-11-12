//! Offline fix-command tests. The fake Cargo records updates and can replace a lockfile.
#![cfg(all(feature = "fix", unix))]

use std::{fs, os::unix::fs::PermissionsExt, process::Command};
use tempfile::TempDir;

struct Fixture(TempDir);

impl Fixture {
    fn new(kind: Option<&str>, patched: bool) -> Self {
        let dir = TempDir::new().unwrap();
        let p = dir.path();
        fs::create_dir_all(p.join("db/crates/example")).unwrap();
        let metadata = kind
            .map(|s| format!("informational = \"{s}\"\n"))
            .unwrap_or_default();
        let versions = if patched { "[\">= 0.2.0\"]" } else { "[]" };
        fs::write(p.join("db/crates/example/RUSTSEC-2024-0001.md"), format!(
            "```toml\n[advisory]\nid = \"RUSTSEC-2024-0001\"\npackage = \"example\"\ndate = \"2024-01-01\"\n{metadata}\n[versions]\npatched = {versions}\n```\n\n# Test advisory\n\nOffline fixture.\n"
        )).unwrap();
        let lock = |version| {
            format!(
                "version = 3\n\n[[package]]\nname = \"example\"\nversion = \"{version}\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n"
            )
        };
        fs::write(p.join("Cargo.lock"), lock("0.1.0")).unwrap();
        fs::write(p.join("fixed.lock"), lock("0.2.0")).unwrap();
        fs::create_dir(p.join(".cargo")).unwrap();
        fs::write(
            p.join(".cargo/audit.toml"),
            "[advisories]\ninformational_warnings = [\"unsound\", \"notice\", \"unmaintained\"]\n",
        )
        .unwrap();
        fs::write(p.join("fake-cargo"), "#!/bin/sh\nprintf '%s\\n' \"$@\" >> \"$FIX_LOG\"\nif [ \"$FIX_FAIL\" = yes ]; then exit 1; fi\nfor arg do if [ \"$arg\" = --dry-run ]; then exit 0; fi; done\nif [ \"$FIX_APPLY\" = yes ]; then cp \"$FIX_LOCK\" Cargo.lock; fi\n").unwrap();
        fs::set_permissions(p.join("fake-cargo"), fs::Permissions::from_mode(0o755)).unwrap();
        Self(dir)
    }

    fn run(&self, args: &[&str], apply: bool, fail: bool) -> std::process::Output {
        let p = self.0.path();
        Command::new(env!("CARGO_BIN_EXE_cargo-audit"))
            .current_dir(p)
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
            .env("CARGO", p.join("fake-cargo"))
            .env("FIX_LOG", p.join("calls"))
            .env("FIX_LOCK", p.join("fixed.lock"))
            .env("FIX_APPLY", if apply { "yes" } else { "no" })
            .env("FIX_FAIL", if fail { "yes" } else { "no" })
            .output()
            .unwrap()
    }

    fn calls(&self) -> String {
        fs::read_to_string(self.0.path().join("calls")).unwrap_or_default()
    }
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn fixes_configured_informational_advisories() {
    for kind in ["unsound", "notice", "unmaintained"] {
        let f = Fixture::new(Some(kind), true);
        let out = f.run(&["fix"], true, false);
        assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
        assert!(
            f.calls().contains("example@0.1.0"),
            "{kind}: {}",
            stderr(&out)
        );
        assert!(
            stderr(&out).contains("Fixed 1 advisories"),
            "{}",
            stderr(&out)
        );
        assert_eq!(
            fs::read(f.0.path().join("Cargo.lock")).unwrap(),
            fs::read(f.0.path().join("fixed.lock")).unwrap()
        );
    }
}

#[test]
fn warning_dry_run_does_not_change_lockfile() {
    let f = Fixture::new(Some("unsound"), true);
    let before = fs::read(f.0.path().join("Cargo.lock")).unwrap();
    let out = f.run(&["fix", "--dry-run"], true, false);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(f.calls().contains("--dry-run"));
    assert_eq!(fs::read(f.0.path().join("Cargo.lock")).unwrap(), before);
}

#[test]
fn ignored_or_disabled_warnings_do_not_trigger_updates() {
    for ignored in [true, false] {
        let f = Fixture::new(Some("unsound"), true);
        if !ignored {
            fs::write(
                f.0.path().join(".cargo/audit.toml"),
                "[advisories]\ninformational_warnings = []\n",
            )
            .unwrap();
        }
        let args = if ignored {
            vec!["--ignore", "RUSTSEC-2024-0001", "fix"]
        } else {
            vec!["fix"]
        };
        let out = f.run(&args, true, false);
        assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
        assert!(f.calls().is_empty());
    }
}

#[test]
fn unpatchable_warning_is_reported_without_running_cargo() {
    let f = Fixture::new(Some("unmaintained"), false);
    let out = f.run(&["fix", "--dry-run"], false, false);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert!(stderr(&out).contains("No patched versions available"));
    assert!(f.calls().is_empty());
}

#[test]
fn warning_that_remains_after_update_is_reported() {
    let f = Fixture::new(Some("notice"), true);
    let out = f.run(&["fix"], false, false);
    assert!(
        stderr(&out).contains("have patched versions but could not be fixed"),
        "{}",
        stderr(&out)
    );
    assert!(!stderr(&out).contains("Fixed 1 advisories"));
}

#[test]
fn vulnerability_fixes_still_work() {
    let f = Fixture::new(None, true);
    let out = f.run(&["fix"], true, false);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(f.calls().contains("example@0.1.0"));
}

#[test]
fn mixed_vulnerabilities_and_warnings_are_both_fixed() {
    let f = Fixture::new(Some("notice"), true);
    let p = f.0.path();
    let advisory = fs::read_to_string(p.join("db/crates/example/RUSTSEC-2024-0001.md"))
        .unwrap()
        .replace("RUSTSEC-2024-0001", "RUSTSEC-2024-0002")
        .replace("informational = \"notice\"", "");
    fs::write(p.join("db/crates/example/RUSTSEC-2024-0002.md"), advisory).unwrap();
    let out = f.run(&["fix"], true, false);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(
        f.calls().lines().filter(|line| *line == "update").count(),
        2
    );
    assert!(
        stderr(&out).contains("Fixed 2 advisories"),
        "{}",
        stderr(&out)
    );
}

#[test]
fn warning_update_spawn_failure_returns_error() {
    let f = Fixture::new(Some("unsound"), true);
    fs::remove_file(f.0.path().join("fake-cargo")).unwrap();
    let out = f.run(&["fix"], true, false);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains("Failed to run `cargo update`"));
}

#[test]
fn explicitly_selected_lockfile_keeps_update_directory() {
    let f = Fixture::new(Some("notice"), true);
    let out = f.run(&["fix", "--file", "Cargo.lock"], true, false);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(f.calls().contains("example@0.1.0"));
    assert_eq!(
        fs::read(f.0.path().join("Cargo.lock")).unwrap(),
        fs::read(f.0.path().join("fixed.lock")).unwrap()
    );
}
