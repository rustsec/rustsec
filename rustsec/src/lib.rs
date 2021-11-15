//! `rustsec`: client library for the RustSec Security Advisory Database
//!
//! This crate is primarily intended for use with the cargo-audit tool:
//!
//! <https://crates.io/crates/cargo-audit>

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/main/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/rustsec/0.25.1"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
mod error;

pub mod advisory;
mod collection;
pub mod database;
pub mod osv;
pub mod report;
pub mod repository;
mod vulnerability;
pub mod warning;

#[cfg(feature = "fix")]
mod fixer;

#[cfg(feature = "git")]
pub mod registry;

pub use cargo_lock::{self, lockfile, package};
pub use fs_err as fs;
pub use platforms;
pub use semver::{self, Version, VersionReq};

pub use crate::{
    advisory::Advisory,
    collection::Collection,
    database::Database,
    error::{Error, ErrorKind, Result},
    report::Report,
    vulnerability::Vulnerability,
    warning::Warning,
};

#[cfg(feature = "fix")]
pub use crate::fixer::Fixer;

#[cfg(feature = "git")]
pub use crate::repository::git::Repository;

// Use BTreeMap and BTreeSet as our map and set types
use std::collections::{btree_map as map, btree_set as set, BTreeMap as Map, BTreeSet as Set};

/// Current version of the RustSec crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
