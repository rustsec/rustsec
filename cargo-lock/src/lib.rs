//! `cargo-lock`: Self-contained `Cargo.lock` parser/serializer with support
//! for both the V1 and V2 (merge-friendly) formats, as well as optional
//! dependency tree analysis features. Used by [RustSec].
//!
//! ## Minimum Supported Rust Version
//!
//! Rust **1.41** or higher.
//!
//! Minimum supported Rust version can be changed in the future, but it will be
//! done with a minor version bump.
//!
//! ## SemVer Policy
//!
//! - MSRV is considered exempt from SemVer as noted above
//! - The `cargo lock` CLI interface is not considered to have a stable interface
//!   and is also exempted from SemVer. We reserve the right to make substantial
//!   changes to it at any time (for now)
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
//! # Command Line Interface
//!
//! This crate provides a `cargo lock` Cargo subcommand which can be installed
//! by running the following:
//!
//! ```text
//! $ cargo install cargo-lock --features cli
//! ```
//!
//! It supports the following subcommands:
//!
//! ### `list`: summarize packages in `Cargo.lock`
//!
//! The `cargo lock list` subcommand (which can be shortened to just
//! `cargo lock` if you prefer) provides a short synopsis of the packages
//! enumerated in `Cargo.lock`:
//!
//! ```text
//! $ cargo lock
//! - autocfg 1.0.0
//! - cargo-lock 4.0.1
//! - fixedbitset 0.2.0
//! - gumdrop 0.8.0
//! - gumdrop_derive 0.8.0
//! - idna 0.2.0
//! - indexmap 1.3.2
//! - matches 0.1.8
//! [...]
//! ```
//!
//! Adding a `-d` (or `--dependencies`) flag will show transitive dependencies:
//!
//! ```text
//! $ cargo lock -d
//! - autocfg 1.0.0
//! - cargo-lock 4.0.1
//!   - gumdrop 0.8.0
//!   - petgraph 0.5.1
//!   - semver 0.10.0
//!   - serde 1.0.116
//!   - toml 0.5.6
//!   - url 2.1.1
//! - fixedbitset 0.2.0
//! - gumdrop 0.8.0
//!   - gumdrop_derive 0.8.0
//! - gumdrop_derive 0.8.0
//!   - proc-macro2 1.0.21
//!   - quote 1.0.3
//!   - syn 1.0.40
//! - idna 0.2.0
//!   - matches 0.1.8
//!   - unicode-bidi 0.3.4
//!   - unicode-normalization 0.1.12
//! [...]
//! ```
//!
//! Adding a `-s` (or `--source`) flag will show source information for each
//! package (when available):
//!
//! ```text
//! - autocfg 1.0.0 (registry+https://github.com/rust-lang/crates.io-index)
//! - cargo-lock 4.0.1
//! - fixedbitset 0.2.0 (registry+https://github.com/rust-lang/crates.io-index)
//! - gumdrop 0.8.0 (registry+https://github.com/rust-lang/crates.io-index)
//! - gumdrop_derive 0.8.0 (registry+https://github.com/rust-lang/crates.io-index)
//! - idna 0.2.0 (registry+https://github.com/rust-lang/crates.io-index)
//! - indexmap 1.3.2 (registry+https://github.com/rust-lang/crates.io-index)
//! [...]
//! ```
//!
//! ### `translate`: convert `Cargo.lock` files between the V1 and V2 formats
//!
//! The `cargo lock translate` subcommand can translate V1 Cargo.lock files to
//! the [V2 format] and vice versa:
//!
//! ```text
//! $ cargo lock translate
//! ```
//!
//! ...will translate Cargo.lock to the V2 format. To translate a V2 Cargo.lock
//! file back to the V1 format, use:
//!
//! ```text
//! $ cargo lock translate -v1
//! ```
//!
//! ### `tree`: provide information for how a dependency is included
//!
//! The `cargo lock tree` subcommand (similar to the `cargo-tree` command)
//! can provide a visualization of the current dependency tree or how a
//! particular dependency is being used in your project, by consulting
//! `Cargo.lock` alone:
//!
//! ```text
//! $ cargo lock tree
//! cargo-lock 4.0.1
//! ├── url 2.1.1
//! │   ├── percent-encoding 2.1.0
//! │   ├── matches 0.1.8
//! │   └── idna 0.2.0
//! │       ├── unicode-normalization 0.1.12
//! │       │   └── smallvec 1.2.0
//! │       ├── unicode-bidi 0.3.4
//! │       │   └── matches 0.1.8
//! │       └── matches 0.1.8
//! ├── toml 0.5.6
//! │   └── serde 1.0.116
//! │       └── serde_derive 1.0.116
//! [...]
//! ```
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
//! [RustSec]: https://rustsec.org/
//! [V2 format]: https://github.com/rust-lang/cargo/pull/7070
//! [`petgraph`]: https://github.com/petgraph/petgraph
//! [`cargo-tree`]: https://github.com/sfackler/cargo-tree

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/main/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/cargo-lock/7.0.1"
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
