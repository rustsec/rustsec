//! `cargo-lock`: Self-contained `Cargo.lock` parser/serializer with support
//! for both the V1 and V2 (merge-friendly) formats, as well as optional
//! dependency tree analysis features. Used by [RustSec].
//!
//! # Usage Example
//!
//! ```
//! use cargo_lock::Lockfile;
//!
//! let lockfile = Lockfile::load("Cargo.lock").unwrap();
//! println!("number of dependencies: {}", lockfile.packages.len());
//! ```
//!
//! # Command Line Interface
//!
//! This crate provides a `cargo lock` Cargo subcommand which can be installed
//! by running the following:
//!
//! ```text
//! $ cargo install cargo-lock
//! ```
//!
//! It supports the following subcommands:
//!
//! ## `translate`: convert `Cargo.lock` files between the V1 and V2 formats
//!
//! The `cargo lock translate` subcommand can translate V1 Cargo.lock files to
//! the [new V2 format] and vice versa:
//!
//! ```text
//! $ cargo lock translate
//! ```
//!
//! ...will translate Cargo.lock to the V2 format. To translate a V2 Cargo.lock
//! file back to the V1 format, use:
//!
//! ```text
//! $ cargo lock translate --v1
//! ```
//!
//! ## `tree`: provide information for how a dependency is included
//!
//! The `cargo lock tree` subcommand (similar to the `cargo-tree` command)
//! can provide a visualization of how a particular dependency is being used
//! in your project, by consulting Cargo.lock alone:
//!
//! ```text
//! $ cargo lock tree syn
//! syn 1.0.14
//! ├── serde_derive 1.0.104
//! │   └── serde 1.0.104
//! │       ├── toml 0.5.6
//! │       │   └── cargo-lock 3.0.0
//! │       ├── semver 0.9.0
//! │       │   └── cargo-lock 3.0.0
//! │       └── cargo-lock 3.0.0
//! └── gumdrop_derive 0.7.0
//!    └── gumdrop 0.7.0
//!        └── cargo-lock 3.0.0
//! ```
//!
//! # Dependency tree API
//!
//! When the `dependency-tree` feature of this crate is enabled, it supports
//! computing a directed graph of the dependency tree expressed in the
//! lockfile, modeled using the [`petgraph`] crate, along with support for
//! printing dependency trees ala the [`cargo-tree`] crate, a CLI intreface
//! for which is provided by the `cargo lock tree` subcommand described above.
//!
//! This same graph representation of a `Cargo.lock` file is programatically
//! available via this crate's API.
//!
//! [RustSec]: https://rustsec.org/
//! [new V2 format]: https://github.com/rust-lang/cargo/pull/7070
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
