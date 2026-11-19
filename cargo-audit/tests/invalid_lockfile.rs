//! Offline CLI regression for an inconsistent lockfile's dependency tree.

use std::{fs, process::Command};
use tempfile::TempDir;

#[test]
fn missing_dependency_is_reported_without_panicking() {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("db/crates")).unwrap();
    fs::create_dir(dir.path().join(".cargo")).unwrap();
    fs::write(
        dir.path().join("Cargo.lock"),
        include_str!("support/missing-dependency/Cargo.lock"),
    )
    .unwrap();

    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_cargo-audit"))
            .current_dir(dir.path())
            .args([
                "audit",
                "--file",
                "Cargo.lock",
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
    };

    for (args, config) in [
        (&[][..], ""),
        (&["--quiet"][..], ""),
        (&[][..], "[output]\nquiet = false\nshow_tree = false\n"),
    ] {
        fs::write(dir.path().join(".cargo/audit.toml"), config).unwrap();
        let output = run(args);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{stderr}");
        assert!(stderr.contains("failed to find dependency: missing 1.0.0"));
        assert!(!stderr.contains("panicked"), "{stderr}");
    }

    for format in ["json", "sarif"] {
        let output = run(&["--format", format]);
        assert_eq!(output.status.code(), Some(0));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        match format {
            "json" => assert_eq!(report["vulnerabilities"]["count"], 0),
            "sarif" => assert_eq!(report["version"], "2.1.0"),
            _ => unreachable!(),
        }
    }
}
