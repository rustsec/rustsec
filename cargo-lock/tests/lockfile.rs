//! Lockfile integration test

// TODO(tarcieri): add more example `Cargo.lock` files which cover more scenarios

use cargo_lock::{metadata, Lockfile, ResolveVersion, Version};

/// Path to a V1 `Cargo.lock` file.
const V1_LOCKFILE_PATH: &str = "tests/examples/Cargo.lock.v1";

/// Path to a V2 `Cargo.lock` file.
const V2_LOCKFILE_PATH: &str = "tests/examples/Cargo.lock.v2";

/// Path to a V3 `Cargo.lock` file.
const V3_LOCKFILE_PATH: &str = "tests/examples/Cargo.lock.v3";

/// Load example V1 `Cargo.lock` file (from the Cargo project itself)
#[test]
fn load_example_v1_lockfile() {
    let lockfile = Lockfile::load(V1_LOCKFILE_PATH).unwrap();

    assert_eq!(lockfile.version, ResolveVersion::V1);
    assert_eq!(lockfile.packages.len(), 141);
    assert_eq!(lockfile.metadata.len(), 136);

    let package = &lockfile.packages[0];
    assert_eq!(package.name.as_ref(), "adler32");
    assert_eq!(package.version, Version::parse("1.0.4").unwrap());

    let metadata_key: metadata::Key =
        "checksum adler32 1.0.4 (registry+https://github.com/rust-lang/crates.io-index)"
            .parse()
            .unwrap();

    let metadata_value = &lockfile.metadata[&metadata_key];
    assert_eq!(
        metadata_value.as_ref(),
        "5d2e7343e7fc9de883d1b0341e0b13970f764c14101234857d2ddafa1cb1cac2"
    );
}

/// Load example V2 `Cargo.lock` file
#[test]
fn load_example_v2_lockfile() {
    let lockfile = Lockfile::load(V2_LOCKFILE_PATH).unwrap();
    assert_eq!(lockfile.version, ResolveVersion::V2);
    assert_eq!(lockfile.packages.len(), 25);
    assert_eq!(lockfile.metadata.len(), 0);
}

/// Load example V3 `Cargo.lock` file
#[test]
fn load_example_v3_lockfile() {
    let lockfile = Lockfile::load(V3_LOCKFILE_PATH).unwrap();
    assert_eq!(lockfile.version, ResolveVersion::V3);
    assert_eq!(lockfile.packages.len(), 25);
    assert_eq!(lockfile.metadata.len(), 0);
}

/// Ensure we can serialize a V2 lockfile as a V1 lockfile
#[test]
fn serialize_v2_to_v1() {
    let mut lockfile = Lockfile::load(V2_LOCKFILE_PATH).unwrap();
    lockfile.version = ResolveVersion::V1;

    let reserialized = lockfile.to_string();
    let lockfile2 = reserialized.parse::<Lockfile>().unwrap();
    assert_eq!(lockfile2.version, ResolveVersion::V1);
    assert_eq!(lockfile2.packages, lockfile.packages);
}

/// Ensure we can serialize a V1 lockfile as a V2 lockfile
#[test]
fn serialize_v1_to_v2() {
    let mut lockfile = Lockfile::load(V1_LOCKFILE_PATH).unwrap();
    lockfile.version = ResolveVersion::V2;

    let reserialized = lockfile.to_string();
    let lockfile2 = reserialized.parse::<Lockfile>().unwrap();
    assert_eq!(lockfile.packages, lockfile2.packages);
}

/// Dependency tree tests
#[cfg(feature = "dependency-tree")]
mod tree {
    use super::{Lockfile, V1_LOCKFILE_PATH, V2_LOCKFILE_PATH};

    /// Compute a dependency graph from a non-trivial example V1 `Cargo.lock`
    #[test]
    fn compute_from_v1_example_lockfile() {
        let tree = Lockfile::load(V1_LOCKFILE_PATH)
            .unwrap()
            .dependency_tree()
            .unwrap();

        assert_eq!(tree.nodes().len(), 141);
    }

    /// Compute a dependency graph from a non-trivial example V2 `Cargo.lock`
    #[test]
    fn compute_from_v2_example_lockfile() {
        let tree = Lockfile::load(V2_LOCKFILE_PATH)
            .unwrap()
            .dependency_tree()
            .unwrap();

        assert_eq!(tree.nodes().len(), 25);
    }
}
