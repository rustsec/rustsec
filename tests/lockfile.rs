/// Lockfile integration test
use cargo_lock::Lockfile;

/// Load our own `Cargo.lock` file
// TODO(tarcieri): lockfile fixtures for a variety of scenarios
#[test]
fn load_our_own_lockfile() {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();

    // TODO(tarcieri): more assertions
    assert!(lockfile.packages.len() > 0);
}
