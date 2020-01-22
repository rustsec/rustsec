//! `cargo-lock`: Self-contained `Cargo.lock` parser/serializer with support
//! for both the V1 and V2 (merge-friendly) formats, as well as optional
//! dependency tree analysis features. Used by [RustSec].
//!
//! # Usage
//!
//! ```
//! use cargo_lock::Lockfile;
//!
//! let lockfile = Lockfile::load("Cargo.lock").unwrap();
//! println!("number of dependencies: {}", lockfile.packages.len());
//! ```
//!
//! # Dependency tree
//!
//! When the `dependency-tree` feature of this crate is enabled, it supports
//! computing a directed graph of the dependency tree expressed in the
//! lockfile, modeled using the [`petgraph`] crate, along with support for
//! printing dependency trees ala the [`cargo-tree`] crate.
//!
//! [RustSec]: https://rustsec.org/
//! [`petgraph`]: https://github.com/petgraph/petgraph
//! [`cargo-tree`]: https://github.com/sfackler/cargo-tree

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/cargo-lock/3.0.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
pub mod error;

pub mod dependency;
pub mod lockfile;
pub mod metadata;
pub mod package;
pub mod patch;

pub use self::{
    dependency::Dependency,
    error::{Error, ErrorKind},
    lockfile::{Lockfile, ResolveVersion},
    metadata::Metadata,
    package::{Checksum, Name, Package, SourceId, Version},
    patch::Patch,
};

/// Use `BTreeMap` for all `Map` types in the crate
use std::collections::BTreeMap as Map;
