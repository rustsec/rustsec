#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::testing::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RUNNER: CmdRunner = CmdRunner::default();
}

/// Run `rustsec-admin check` against a freshly fetched advisory DB repo
#[test]
fn check_advisory_db() {
    // Fetch the advisory database
    rustsec::Repository::fetch_default_repo().unwrap();

    let mut runner = RUNNER.clone();

    runner
        .arg("check")
        .arg(&rustsec::Repository::default_path())
        .capture_stdout()
        .status()
        .expect_success();
}
