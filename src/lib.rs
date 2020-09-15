//! `rustsec`: client library for the RustSec Security Advisory Database
//!
//! This crate is primarily intended for use with the cargo-audit tool:
//!
//! <https://crates.io/crates/cargo-audit>

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/rustsec/0.21.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
pub mod error;

pub mod advisory;
pub mod collection;
pub mod database;
#[cfg(feature = "fix")]
pub mod fixer;
#[cfg(feature = "fetch")]
pub mod registry;
pub mod report;
pub mod repository;
pub mod version;
pub mod vulnerability;
pub mod warning;

// Re-export the `cargo-lock` crate
pub use cargo_lock;

// Re-export the `platforms` crate
pub use platforms;

pub use crate::{
    advisory::Advisory,
    collection::Collection,
    database::Database,
    error::{Error, ErrorKind},
    report::Report,
    repository::Repository,
    version::{Version, VersionReq},
    vulnerability::Vulnerability,
    warning::Warning,
};

#[cfg(feature = "fetch")]
pub use crate::repository::git::GitRepository;

pub use cargo_lock::{lockfile, package};

// Use BTreeMap and BTreeSet as our map and set types
use std::collections::{btree_map as map, btree_set as set, BTreeMap as Map, BTreeSet as Set};

/// Current version of the RustSec crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
