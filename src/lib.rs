//! Common Vulnerability Scoring System.
//!
//! The [cvss::v3::Base](v3/base/struct.Base.html) type provides the main
//! functionality presently implemented by this crate, namely: support for
//! parsing, serializing, and scoring `CVSS:3.0` and `CVSS:3.1`
//! Base Metric Group vector strings as described in the CVSS v3.1 Specification:
//!
//! <https://www.first.org/cvss/specification-document>
//!
//! Serde support is available through the optional `serde` Cargo feature.

// TODO(tarcieri): other CVSS versions, CVSS v3.1 Temporal and Environmental Groups

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustSec/logos/master/rustsec-logo-lg.png",
    html_root_url = "https://docs.rs/cvss/0.3.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
pub mod error;
pub mod severity;

#[cfg(feature = "v3")]
pub mod v3;

pub use self::severity::Severity;

/// Prefix used by all CVSS strings
pub const PREFIX: &str = "CVSS";
