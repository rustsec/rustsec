//! All Tier 3 platforms supported by Rust. Sourced from:
//!
//! <https://forge.rust-lang.org/platform-support.html>
//!
//! Tier 3 platforms are those which the Rust codebase has support for, but
//! which are not built or tested automatically, and may not work.
//! Official builds are not available.

use crate::{
    platform::{Platform, Tier},
    target::{Arch, Env, OS},
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

/// `thumbv6m-none-eabi`: Bare Cortex-M0, M0+, M1
pub const THUMBV6M_NONE_EABI: Platform = Platform {
    target_triple: "thumbv6m-none-eabi",
    target_arch: Arch::THUMBV6,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Three,
};

/// `thumbv7em-none-eabi`:	Bare Cortex-M4, M7
pub const THUMBV7EM_NONE_EABI: Platform = Platform {
    target_triple: "thumbv7em-none-eabi",
    target_arch: Arch::THUMBV7,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Three,
};

/// `thumbv7em-none-eabihf`: Bare Cortex-M4F, M7F, FPU, hardfloat
pub const THUMBV7EM_NONE_EABIHF: Platform = Platform {
    target_triple: "thumbv7em-none-eabihf",
    target_arch: Arch::THUMBV7,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Three,
};

/// `thumbv7m-none-eabi`: Bare Cortex-M3
pub const THUMBV7M_NONE_EABI: Platform = Platform {
    target_triple: "thumbv7m-none-eabi",
    target_arch: Arch::THUMBV7,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Three,
};

/// `x86_64-fortanix-unknown-sgx`: 	Fortanix ABI for 64-bit Intel SGX
pub const X86_64_FORTANIX_UNKNOWN_SGX: Platform = Platform {
    target_triple: "x86_64-fortanix-unknown-sgx",
    target_arch: Arch::X86_64,
    target_os: OS::Unknown,
    target_env: Some(Env::SGX),
    tier: Tier::Three,
};

/// `x86_64-unknown-bitrig`: 64-bit Bitrig
pub const X86_64_UNKNOWN_BITRIG: Platform = Platform {
    target_triple: "x86_64-unknown-bitrig",
    target_arch: Arch::X86_64,
    target_os: OS::Bitrig,
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
