//! Rust platform registry: provides programmatic access to information about valid Rust platforms
//!
//! This crate provides an interface to the platform data canonically sourced
//! from the Rust compiler:
//!
//! <https://doc.rust-lang.org/nightly/rustc/platform-support.html>
//!
//! ## Minimum Supported Rust Version
//!
//! Rust **1.40** or higher.
//!
//! Minimum supported Rust version can be changed in the future, but it will be
//! done with a minor version bump.

#![no_std]
#![doc(html_root_url = "https://docs.rs/platforms/1.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, unused_qualifications, rust_2018_idioms)]

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod error;
pub mod platform;
pub mod target;

pub use crate::{
    error::Error,
    platform::{Platform, Tier, ALL_PLATFORMS},
    target::{TARGET_ARCH, TARGET_ENV, TARGET_OS},
};

#[cfg(feature = "std")]
pub use crate::platform::PlatformReq;

/// Find a Rust platform by its "target triple", e.g. `i686-apple-darwin`
pub fn find(target_triple: &str) -> Option<&'static Platform> {
    ALL_PLATFORMS
        .iter()
        .find(|platform| platform.target_triple == target_triple)
}

/// Attempt to guess the current `Platform`. May give inaccurate results.
pub fn guess_current() -> Option<&'static Platform> {
    ALL_PLATFORMS.iter().find(|platform| {
        platform.target_arch == TARGET_ARCH
            && platform.target_env == TARGET_ENV
            && platform.target_os == TARGET_OS
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn guesses_current() {
        assert!(super::guess_current().is_some());
    }
}
