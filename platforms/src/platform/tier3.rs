//! All Tier 3 platforms supported by Rust. Sourced from:
//!
//! <https://doc.rust-lang.org/nightly/rustc/platform-support.html>
//!
//! Tier 3 platforms are those which the Rust codebase has support for, but
//! which are not built or tested automatically, and may not work.
//! Official builds are not available.

use crate::{
    platform::{Platform, Tier},
    target::{Arch, Env, OS},
};

/// `armv7-apple-ios`: ARMv7 iOS, Cortex-a8
pub const ARMV7_APPLE_IOS: Platform = Platform {
    target_triple: "armv7-apple-ios",
    target_arch: Arch::ARM,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Three,
};

/// `armv7s-apple-ios`: ARMv7 iOS, Cortex-a9
pub const ARMV7S_APPLE_IOS: Platform = Platform {
    target_triple: "armv7s-apple-ios",
    target_arch: Arch::ARM,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Three,
};

/// `i386-apple-ios`: 32-bit x86 iOS
pub const I386_APPLE_IOS: Platform = Platform {
    target_triple: "i386-apple-ios",
    target_arch: Arch::X86,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Three,
};

/// `i686-apple-darwin`: 32-bit OSX (10.7+, Lion+)
pub const I686_APPLE_DARWIN: Platform = Platform {
    target_triple: "i686-apple-darwin",
    target_arch: Arch::X86,
    target_os: OS::MacOS,
    target_env: None,
    tier: Tier::Three,
};

/// `i686-unknown-haiku`: 32-bit Haiku
pub const I686_UNKNOWN_HAIKU: Platform = Platform {
    target_triple: "i686-unknown-haiku",
    target_arch: Arch::X86,
    target_os: OS::Haiku,
    target_env: None,
    tier: Tier::Three,
};

/// `i686-unknown-netbsd`: NetBSD/i386 with SSE2
pub const I686_UNKNOWN_NETBSD: Platform = Platform {
    target_triple: "i686-unknown-netbsd",
    target_arch: Arch::X86,
    target_os: OS::NetBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `mips-unknown-linux-uclibc`: MIPS Linux with uClibc
pub const MIPS_UNKNOWN_LINUX_UCLIBC: Platform = Platform {
    target_triple: "mips-unknown-linux-uclibc",
    target_arch: Arch::MIPS,
    target_os: OS::Linux,
    target_env: Some(Env::uClibc),
    tier: Tier::Three,
};

/// `mipsel-unknown-linux-uclibc`: MIPS (LE) Linux with uClibc
pub const MIPSEL_UNKNOWN_LINUX_UCLIBC: Platform = Platform {
    target_triple: "mipsel-unknown-linux-uclibc",
    target_arch: Arch::MIPS,
    target_os: OS::Linux,
    target_env: Some(Env::uClibc),
    tier: Tier::Three,
};

/// `msp430-none-elf`: 16-bit MSP430 microcontrollers
pub const MSP430_NONE_ELF: Platform = Platform {
    target_triple: "msp430-none-elf",
    target_arch: Arch::MSP430,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Three,
};

/// `sparc64-unknown-netbsd`: NetBSD/sparc64
pub const SPARC64_UNKNOWN_NETBSD: Platform = Platform {
    target_triple: "sparc64-unknown-netbsd",
    target_arch: Arch::SPARC64,
    target_os: OS::NetBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `x86_64-sun-solaris`: Deprecated target for 64-bit Solaris 10/11, illumos
pub const X86_64_SUN_SOLARIS: Platform = Platform {
    target_triple: "x86_64-sun-solaris",
    target_arch: Arch::X86_64,
    target_os: OS::Solaris,
    target_env: None,
    tier: Tier::Three,
};

/// `x86_64-unknown-dragonfly`: 64-bit DragonFlyBSD
pub const X86_64_UNKNOWN_DRAGONFLY: Platform = Platform {
    target_triple: "x86_64-unknown-dragonfly",
    target_arch: Arch::X86_64,
    target_os: OS::Dragonfly,
    target_env: None,
    tier: Tier::Three,
};

/// `x86_64-unknown-haiku`: 64-bit Haiku
pub const X86_64_UNKNOWN_HAIKU: Platform = Platform {
    target_triple: "x86_64-unknown-haiku",
    target_arch: Arch::X86_64,
    target_os: OS::Haiku,
    target_env: None,
    tier: Tier::Three,
};

/// `x86_64-unknown-openbsd`: 64-bit OpenBSD
pub const X86_64_UNKNOWN_OPENBSD: Platform = Platform {
    target_triple: "x86_64-unknown-openbsd",
    target_arch: Arch::X86_64,
    target_os: OS::OpenBSD,
    target_env: None,
    tier: Tier::Three,
};
