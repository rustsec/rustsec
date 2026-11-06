use rustsec::Database;
use std::fs;

#[test]
fn placeholder_filename_error_explains_multiple_advisories() {
    let dir = tempfile::tempdir().unwrap();
    let package_dir = dir.path().join("crates/base");
    fs::create_dir_all(&package_dir).unwrap();

    let advisory = include_str!("support/example_advisory_v3.md")
        .replace("RUSTSEC-2001-2101", "RUSTSEC-0000-0000");
    let path = package_dir.join("RUSTSEC-0000-0000-decompression.md");
    fs::write(&path, &advisory).unwrap();

    let error = Database::open(dir.path()).unwrap_err().to_string();
    assert!(
        error.contains("RUSTSEC-0000-0000.decompression.md"),
        "{error}"
    );
    assert!(error.contains("dot, not a hyphen"), "{error}");
    assert!(error.contains("one PR"), "{error}");
    assert!(error.contains("one commit per advisory"), "{error}");
    assert!(error.contains("Keep the advisory id"), "{error}");

    fs::rename(
        &path,
        package_dir.join("RUSTSEC-0000-0000.decompression.md"),
    )
    .unwrap();
    fs::write(package_dir.join("RUSTSEC-0000-0000.brotli.md"), &advisory).unwrap();
    Database::open(dir.path()).unwrap();
}
