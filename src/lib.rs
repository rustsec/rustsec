//! Audit Cargo.lock files for crates containing security vulnerabilities.
//!
//! `cargo audit` is a Cargo subcommand. Install it using the following:
//!
//! ```text
//! $ cargo install cargo-audit
//! ```
//!
//! Then run `cargo audit` in the toplevel directory of any crate or workspace.
//!
//! If you wish to consume its core functionality as a library, see the
//! documentation for the `rustsec` crate:
//!
//! <https://docs.rs/rustsec/>

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/cargo-audit/0.9.0-beta1"
)]

#[macro_use]
extern crate abscissa_core;

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
mod prelude;
pub mod tree;
