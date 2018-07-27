//! Rust platform registry: provides programmatic access to information about valid Rust platforms
//!
//! This crate provides an interface to the platform data available at Rust Forge:
//!
//! <https://forge.rust-lang.org/platform-support.html>

#![crate_name = "platforms"]
#![crate_type = "lib"]
#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/platforms/0.1.0")]
#![no_std]

#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "std")]
extern crate std;

/// Error types
pub(crate) mod error;

/// Rust platform types
pub mod platform;

/// Rust target types
pub mod target;

pub use error::Error;
#[cfg(feature = "std")]
pub use platform::PlatformReq;
pub use platform::{Platform, ALL_PLATFORMS};
pub use target::{TARGET_ARCH, TARGET_ENV, TARGET_OS};

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
