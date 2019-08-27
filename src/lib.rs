//! rustsec: Client library for the `RustSec` security advisory database
//!
//! This crate is primarily intended for use with the cargo-audit tool:
//!
//! <https://crates.io/crates/cargo-audit>

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/rustsec/0.12.1"
)]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

#[macro_use]
pub mod error;

pub mod advisory;
pub mod db;
pub mod lockfile;
pub mod package;
pub mod repository;
pub mod version;
pub mod vulnerability;

pub use crate::{
    advisory::*, db::*, error::*, lockfile::*, package::*, repository::*, version::*,
    vulnerability::*,
};
