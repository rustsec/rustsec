//! `cargo-lock`: Self-contained `Cargo.lock` parser with optional dependency
//! graph analysis.
//!
//! When the `dependency-graph` feature of this crate is enabled, it supports
//! computing a directed graph of the dependency tree expressed in the lockfile,
//! modeled using the [`petgraph`] crate.
//!
//! [`petgraph`]: https://github.com/petgraph/petgraph

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/cargo-lock/0.1.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
pub mod error;

#[cfg(feature = "dependency-graph")]
pub mod dependency_graph;
pub mod lockfile;
pub mod metadata;
pub mod package;

pub use self::{
    error::{Error, ErrorKind},
    lockfile::Lockfile,
    package::Package,
};

#[cfg(feature = "dependency-graph")]
pub use self::dependency_graph::DependencyGraph;
