//! All Tier 1 platforms supported by Rust. Sourced from:
//!
//! <https://doc.rust-lang.org/nightly/rustc/platform-support.html>
//!
//! Tier 1 platforms can be thought of as “guaranteed to work”.
//! Specifically they will each satisfy the following requirements:
//!
//! - Official binary releases are provided for the platform.
//! - Automated testing is set up to run tests for the platform.
//! - Landing changes to the rust-lang/rust repository’s master branch
//!   is gated on tests passing.
//! - Documentation for how to use and how to build the platform is available.

use crate::{
    platform::{Platform, Tier},
    target::{Arch, Env, OS},
};

/// `aarch64-unknown-linux-gnu`: ARM64 Linux
pub const AARCH64_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "aarch64-unknown-linux-gnu",
    target_arch: Arch::AArch64,
    target_os: OS::Linux,
    target_env: Some(Env::Gnu),
    tier: Tier::One,
};

/// `i686-pc-windows-gnu`: 32-bit MinGW (Windows 7+)
pub const I686_PC_WINDOWS_GNU: Platform = Platform {
    target_triple: "i686-pc-windows-gnu",
    target_arch: Arch::X86,
    target_os: OS::Windows,
    target_env: Some(Env::Gnu),
    tier: Tier::One,
};

/// `i686-pc-windows-msvc`: 32-bit MSVC (Windows 7+)
pub const I686_PC_WINDOWS_MSVC: Platform = Platform {
    target_triple: "i686-pc-windows-msvc",
    target_arch: Arch::X86,
    target_os: OS::Windows,
    target_env: Some(Env::Msvc),
    tier: Tier::One,
};

/// `i686-unknown-linux-gnu`: 32-bit Linux (2.6.18+)
pub const I686_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "i686-unknown-linux-gnu",
    target_arch: Arch::X86,
    target_os: OS::Linux,
    target_env: Some(Env::Gnu),
    tier: Tier::One,
};

/// `x86_64-apple-darwin`: 64-bit OSX (10.7+, Lion+)
pub const X86_64_APPLE_DARWIN: Platform = Platform {
    target_triple: "x86_64-apple-darwin",
    target_arch: Arch::X86_64,
    target_os: OS::MacOS,
    target_env: None,
    tier: Tier::One,
};

/// `x86_64-pc-windows-gnu`: 64-bit MinGW (Windows 7+)
pub const X86_64_PC_WINDOWS_GNU: Platform = Platform {
    target_triple: "x86_64-pc-windows-gnu",
    target_arch: Arch::X86_64,
    target_os: OS::Windows,
    target_env: Some(Env::Gnu),
    tier: Tier::One,
};

/// `x86_64-pc-windows-msvc`: 64-bit MSVC (Windows 7+)
pub const X86_64_PC_WINDOWS_MSVC: Platform = Platform {
    target_triple: "x86_64-pc-windows-msvc",
    target_arch: Arch::X86_64,
    target_os: OS::Windows,
    target_env: Some(Env::Msvc),
    tier: Tier::One,
};

/// `x86_64-unknown-linux-gnu`: 64-bit Linux (2.6.18+)
pub const X86_64_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "x86_64-unknown-linux-gnu",
    target_arch: Arch::X86_64,
    target_os: OS::Linux,
    target_env: Some(Env::Gnu),
    tier: Tier::One,
};
