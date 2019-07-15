//! rustsec: Client library for the `RustSec` security advisory database
//!
//! This crate is primarily intended for use with the cargo-audit tool:
//!
//! <https://crates.io/crates/cargo-audit>

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/rustsec/0.11.0"
)]

#[macro_use]
pub mod error;

pub mod advisory;
pub mod db;
pub mod lockfile;
pub mod package;
pub mod repository;
pub mod vulnerability;

pub use crate::{
    advisory::*, db::*, error::*, lockfile::*, package::*, repository::*, vulnerability::*,
};
