//! rustsec: Client library for the `RustSec` security advisory database
//!
//! This crate is primarily intended for use with the cargo-audit tool:
//!
//! <https://crates.io/crates/cargo-audit>

#![crate_name = "rustsec"]
#![crate_type = "lib"]
#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/rustsec/0.6.0")]

#[cfg(feature = "chrono")]
extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate git2;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
pub mod error;

pub mod advisory;
pub mod db;
pub mod lockfile;
pub mod package;
pub mod repository;
pub mod vulnerability;

pub use advisory::*;
pub use db::*;
pub use error::*;
pub use lockfile::*;
pub use package::*;
pub use vulnerability::*;
