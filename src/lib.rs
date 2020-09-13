//! Rust platform registry: provides programmatic access to information about valid Rust platforms
//!
//! This crate provides an interface to the platform data available at Rust Forge:
//!
//! <https://forge.rust-lang.org/platform-support.html>
//!
//! ## Minimum Supported Rust Version
//!
//! Rust **1.41** or higher.
//!
//! Minimum supported Rust version can be changed in the future, but it will be
//! done with a minor version bump.

#![no_std]
#![doc(html_root_url = "https://docs.rs/platforms/0.2.1")]
#![forbid(unsafe_code)]
#![warn(missing_docs, unused_qualifications)]

#[cfg(feature = "std")]
extern crate std;

/// Error types
pub(crate) mod error;

/// Rust platform types
pub mod platform;

/// Rust target types
pub mod target;

#[cfg(feature = "std")]
pub use crate::platform::PlatformReq;
pub use crate::{
    error::Error,
    platform::{Platform, Tier, ALL_PLATFORMS},
    target::{TARGET_ARCH, TARGET_ENV, TARGET_OS},
};

/// Find a Rust platform by its "target triple", e.g. `i686-apple-darwin`
pub fn find<S: AsRef<str>>(target_triple: S) -> Option<&'static Platform> {
    ALL_PLATFORMS
        .iter()
        .find(|platform| platform.target_triple == target_triple.as_ref())
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
