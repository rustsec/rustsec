#![cfg(feature = "binary-scanning")]

mod support;

use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    process::{Command, Output},
    str::from_utf8,
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
    let mut runner = support::cargo_audit();
    runner
        .arg("audit")
        .arg("--color=never")
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

#[test]
fn affected_functions() {
    let binary_path = binaries_dir().join("binary-with-affected-functions");
    let output = run(cmd_runner().arg(binary_path));
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reports = read_stdout(&output.stdout);

    // Verify that all of the expected affected functions were reported.
    for (id, function_paths) in EXPECTED_FUNCTION_PATHS {
        let report = reports
            .iter()
            .find(|map| map.get("ID").unwrap() == id)
            .unwrap_or_else(|| panic!("failed to find {id}"));
        let affected = report
            .get("Affected")
            .unwrap_or_else(|| panic!("{id} has no 'Affected' line"));
        assert_eq!(function_paths.join(", "), *affected);
    }

    // Verify that each report with an "Affected" line was expected.
    for report in reports {
        if !report.contains_key("Affected") {
            continue;
        }
        let id = report.get("ID").unwrap();
        assert!(
            EXPECTED_FUNCTION_PATHS.iter().any(|(key, _)| key == id),
            "{id} has unexpected 'Affected' line"
        );
    }
}

fn read_stdout(stdout: &[u8]) -> BTreeSet<BTreeMap<String, String>> {
    let s = from_utf8(stdout).unwrap();

    let mut reports = BTreeSet::new();
    let mut report = BTreeMap::new();
    for line in s.lines() {
        let Some(index) = line.as_bytes().iter().position(|&x| x == b':') else {
            continue;
        };
        let key = &line[..index];
        let value = line[index + 1..].trim_start();
        // Assume "Crate:" starts a new report.
        if key == "Crate" && !report.is_empty() {
            reports.insert(report);
            report = BTreeMap::new();
        }
        report.insert(key.to_owned(), value.to_owned());
    }
    if !report.is_empty() {
        reports.insert(report);
    }
    reports
}

#[rustfmt::skip]
const EXPECTED_FUNCTION_PATHS: &[(&str, &[&str])] = &[
    ("RUSTSEC-2019-0036", &["failure::Fail::__private_get_type_id__"]),
    ("RUSTSEC-2020-0071", &["time::OffsetDateTime::now_local", "time::OffsetDateTime::try_now_local", "time::UtcOffset::current_local_offset", "time::UtcOffset::local_offset_at", "time::UtcOffset::try_current_local_offset", "time::UtcOffset::try_local_offset_at"]),
    ("RUSTSEC-2020-0075", &["branca::Branca::decode", "branca::decode"]),
    ("RUSTSEC-2021-0041", &["parse_duration::parse"]),
    ("RUSTSEC-2022-0004", &["rustc_serialize::json::Json::from_str"]),
    ("RUSTSEC-2022-0067", &["lzf::compress", "lzf::decompress"]),
    ("RUSTSEC-2023-0032", &["ntru::types::PrivateKey::export", "ntru::types::PublicKey::export"]),
    ("RUSTSEC-2023-0054", &["mail_internals::utils::vec_insert_bytes"]),
    ("RUSTSEC-2024-0018", &["crayon::utils::object_pool::ObjectPool<H,T>::free"]),
    ("RUSTSEC-2024-0020", &["whoami::realname", "whoami::realname_os", "whoami::username", "whoami::username_os"]),
    ("RUSTSEC-2024-0360", &["xmp_toolkit::XmpFile::close"]),
    ("RUSTSEC-2024-0401", &["zlib_rs::inflate::inflate"]),
    ("RUSTSEC-2024-0404", &["anstream::adapter::strip_str"]),
    ("RUSTSEC-2024-0442", &["wasmtime_jit_debug::perf_jitdump::JitDumpFile::dump_code_load_record"]),
    ("RUSTSEC-2025-0027", &["mp3_metadata::read_from_slice"]),
    ("RUSTSEC-2025-0113", &["shaman::cryptoutil::read_u32v_be", "shaman::cryptoutil::read_u32v_le", "shaman::cryptoutil::read_u64v_be", "shaman::cryptoutil::read_u64v_le", "shaman::cryptoutil::write_u32v_le", "shaman::cryptoutil::write_u64v_le"]),
    ("RUSTSEC-2025-0131", &["rtvm_interpreter::Interpreter::program_counter"]),
    ("RUSTSEC-2025-0136", &["sequoia_openpgp::crypto::ecdh::aes_key_unwrap"]),
    ("RUSTSEC-2025-0137", &["ruint::algorithms::div::reciprocal_mg10"]),
    ("RUSTSEC-2025-0140", &["gix_date::parse::TimeBuf::as_str"]),
    ("RUSTSEC-2026-0097", &["rand::rng"]),
];
