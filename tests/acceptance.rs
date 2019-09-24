//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.
//!
//! Modify and/or delete these as you see fit to test the specific needs of
//! your application.
//!
//! For more information, see:
//! <https://docs.rs/abscissa_core/latest/abscissa_core/testing/index.html>

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::testing::prelude::*;
use lazy_static::lazy_static;
use rustsec_admin::config::AdminConfig;

lazy_static! {
    /// Executes your application binary via `cargo run`.
    ///
    /// Storing this value in a `lazy_static!` ensures that all instances of
    /// the runner acquire a mutex when executing commands and inspecting
    /// exit statuses, serializing what would otherwise be multithreaded
    /// invocations as `cargo test` executes tests in parallel by default.
    pub static ref RUNNER: CmdRunner = CmdRunner::default();
}

/// Use `AdminConfig::default()` value if no config or args
#[test]
fn start_no_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner.arg("start").capture_stdout().run();
    cmd.stdout().expect_line("Hello, world!");
    cmd.wait().unwrap().expect_success();
}

/// Use command-line argument value
#[test]
fn start_with_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner
        .args(&["start", "acceptance", "test"])
        .capture_stdout()
        .run();

    cmd.stdout().expect_line("Hello, acceptance test!");
    cmd.wait().unwrap().expect_success();
}

/// Use configured value
#[test]
fn start_with_config_no_args() {
    let mut config = AdminConfig::default();
    config.hello.recipient = "configured recipient".to_owned();
    let expected_line = format!("Hello, {}!", &config.hello.recipient);

    let mut runner = RUNNER.clone();
    let mut cmd = runner.config(&config).arg("start").capture_stdout().run();
    cmd.stdout().expect_line(&expected_line);
    cmd.wait().unwrap().expect_success();
}

/// Override configured value with command-line argument
#[test]
fn start_with_config_and_args() {
    let mut config = AdminConfig::default();
    config.hello.recipient = "configured recipient".to_owned();

    let mut runner = RUNNER.clone();
    let mut cmd = runner
        .config(&config)
        .args(&["start", "acceptance", "test"])
        .capture_stdout()
        .run();

    cmd.stdout().expect_line("Hello, acceptance test!");
    cmd.wait().unwrap().expect_success();
}

/// Example of a test which matches a regular expression
#[test]
fn version_no_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner.arg("version").capture_stdout().run();
    cmd.stdout().expect_regex(r"\Arustsec-admin [\d\.\-]+\z");
}
