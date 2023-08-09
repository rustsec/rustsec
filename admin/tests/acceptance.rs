#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use std::time::Duration;

use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;
use rustsec::repository::git;

pub static RUNNER: Lazy<CmdRunner> = Lazy::new(CmdRunner::default);

const LOCK_WAIT_MINUTES: u64 = 2;

/// Run `rustsec-admin lint` against a freshly fetched advisory DB repo
#[test]
fn lint_advisory_db() {
    // Fetch the advisory database
    git::Repository::fetch_default_repo(Duration::from_secs(LOCK_WAIT_MINUTES * 60)).unwrap();
    let mut runner = RUNNER.clone();

    runner
        .arg("lint")
        .arg("--skip-namecheck")
        .arg("rustdecimal")
        .arg(&git::Repository::default_path())
        .capture_stdout()
        .status()
        .expect_success();
}
