//! `cargo-lock`: Self-contained Cargo.lock parser with optional dependency
//! graph analysis.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/cargo-lock/0.0.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
pub mod error;

pub mod lockfile;
pub mod metadata;
pub mod package;
