//! rustsec: Client library for the `RustSec` security advisory database
//!
//! This crate is primarily intended for use with the cargo-audit tool:
//!
//! <https://crates.io/crates/cargo-audit>

#![crate_name = "rustsec"]
#![crate_type = "lib"]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate reqwest;
extern crate semver;
extern crate toml;

#[macro_use]
pub mod error;

pub mod advisory;
pub mod db;
pub mod lockfile;
mod util;

pub use db::AdvisoryDatabase;
pub use lockfile::Lockfile;

/// URL where the TOML file containing the advisory database is located
pub const ADVISORY_DB_URL: &str =
    "https://raw.githubusercontent.com/RustSec/advisory-db/master/Advisories.toml";
