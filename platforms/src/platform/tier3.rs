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

/// `aarch64-apple-ios-macabi`: Apple Catalyst on ARM64
pub const AARCH64_APPLE_IOS_MACABI: Platform = Platform {
    target_triple: "aarch64-apple-ios-macabi",
    target_arch: Arch::AArch64,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-apple-ios-sim`: Apple iOS Simulator on ARM64
pub const AARCH64_APPLE_IOS_SIM: Platform = Platform {
    target_triple: "aarch64-apple-ios-sim",
    target_arch: Arch::AArch64,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-apple-tvos`: ARM64 tvOS
pub const AARCH64_APPLE_TVOS: Platform = Platform {
    target_triple: "aarch64-apple-tvos",
    target_arch: Arch::AArch64,
    target_os: OS::TvOS,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-unknown-freebsd`: ARM64 FreeBSD
pub const AARCH64_UNKNOWN_FREEBSD: Platform = Platform {
    target_triple: "aarch64-unknown-freebsd",
    target_arch: Arch::AArch64,
    target_os: OS::FreeBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-unknown-hermit`
pub const AARCH64_UNKNOWN_HERMIT: Platform = Platform {
    target_triple: "aarch64-unknown-hermit",
    target_arch: Arch::AArch64,
    target_os: OS::Hermit,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-unknown-linux-gnu_ilp32`: ARM64 Linux (ILP32 ABI)
pub const AARCH64_UNKNOWN_LINUX_GNU_ILP32: Platform = Platform {
    target_triple: "aarch64-unknown-linux-gnu_ilp32",
    target_arch: Arch::AArch64,
    target_os: OS::Linux,
    target_env: Some(Env::Gnu),
    tier: Tier::Three,
};

/// `aarch64-unknown-netbsd`
pub const AARCH64_UNKNOWN_NETBSD: Platform = Platform {
    target_triple: "aarch64-unknown-netbsd",
    target_arch: Arch::AArch64,
    target_os: OS::NetBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-unknown-openbsd`: ARM64 OpenBSD
pub const AARCH64_UNKNOWN_OPENBSD: Platform = Platform {
    target_triple: "aarch64-unknown-openbsd",
    target_arch: Arch::AArch64,
    target_os: OS::OpenBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-unknown-redox`: ARM64 Redox OS
pub const AARCH64_UNKNOWN_REDOX: Platform = Platform {
    target_triple: "aarch64-unknown-redox",
    target_arch: Arch::AArch64,
    target_os: OS::Redox,
    target_env: None,
    tier: Tier::Three,
};

/// `aarch64-uwp-windows-msvc`
pub const AARCH64_UWP_WINDOWS_MSVC: Platform = Platform {
    target_triple: "aarch64-uwp-windows-msvc",
    target_arch: Arch::AArch64,
    target_os: OS::Windows,
    target_env: Some(Env::Msvc),
    tier: Tier::Three,
};

/// `aarch64-wrs-vxworks`
pub const AARCH64_WRS_VXWORKS: Platform = Platform {
    target_triple: "aarch64-wrs-vxworks",
    target_arch: Arch::AArch64,
    target_os: OS::VxWorks,
    target_env: Some(Env::Gnu),
    tier: Tier::Three,
};

/// `aarch64_be-unknown-linux-gnu_ilp32`: ARM64 Linux (big-endian, ILP32 ABI)
pub const AARCH64_BE_UNKNOWN_LINUX_GNU_ILP32: Platform = Platform {
    target_triple: "aarch64_be-unknown-linux-gnu_ilp32",
    target_arch: Arch::AArch64,
    target_os: OS::Linux,
    target_env: Some(Env::Gnu),
    tier: Tier::Three,
};

/// `aarch64_be-unknown-linux-gnu`: ARM64 Linux (big-endian)
pub const AARCH64_BE_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "aarch64_be-unknown-linux-gnu",
    target_arch: Arch::AArch64,
    target_os: OS::Linux,
    target_env: Some(Env::Gnu),
    tier: Tier::Three,
};

/// `armv4t-unknown-linux-gnueabi`
pub const ARMV4T_UNKNOWN_LINUX_GNUEABI: Platform = Platform {
    target_triple: "armv4t-unknown-linux-gnueabi",
    target_arch: Arch::Arm,
    target_os: OS::Linux,
    target_env: Some(Env::Gnu),
    tier: Tier::Three,
};

/// `armv5te-unknown-linux-uclibceabi`: ARMv5TE Linux with uClibc
pub const ARMV5T_UNKNOWN_LINUX_UCLIBCEABI: Platform = Platform {
    target_triple: "armv5te-unknown-linux-uclibceabi",
    target_arch: Arch::Arm,
    target_os: OS::Linux,
    target_env: Some(Env::UClibc),
    tier: Tier::Three,
};

/// `armv6-unknown-freebsd`: ARMv6 FreeBSD
pub const ARMV6_UNKNOWN_FREEBSD: Platform = Platform {
    target_triple: "armv6-unknown-freebsd",
    target_arch: Arch::Arm,
    target_os: OS::FreeBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `armv6-unknown-netbsd-eabihf`
pub const ARMV6_UNKNOWN_NETBSD_EABIHF: Platform = Platform {
    target_triple: "armv6-unknown-netbsd-eabihf",
    target_arch: Arch::Arm,
    target_os: OS::NetBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `armv7-apple-ios`: ARMv7 iOS, Cortex-a8
pub const ARMV7_APPLE_IOS: Platform = Platform {
    target_triple: "armv7-apple-ios",
    target_arch: Arch::Arm,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Three,
};

/// `armv7-unknown-freebsd`: ARMv7 FreeBSD
pub const ARMV7_UNKNOWN_FREEBSD: Platform = Platform {
    target_triple: "armv7-unknown-freebsd",
    target_arch: Arch::Arm,
    target_os: OS::FreeBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `armv7-unknown-netbsd-eabihf`
pub const ARMV7_UNKNOWN_NETBSD_EABIHF: Platform = Platform {
    target_triple: "armv7-unknown-netbsd-eabihf",
    target_arch: Arch::Arm,
    target_os: OS::NetBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `armv7-wrs-vxworks-eabihf`
pub const ARMV7_WRS_VXWORKS_EABIHF: Platform = Platform {
    target_triple: "armv7-wrs-vxworks-eabihf",
    target_arch: Arch::Arm,
    target_os: OS::VxWorks,
    target_env: Some(Env::Gnu),
    tier: Tier::Three,
};

/// `armv7a-none-eabihf`: ARM Cortex-A, hardfloat
pub const ARMV7A_NONE_EABIHF: Platform = Platform {
    target_triple: "armv7a-none-eabihf",
    target_arch: Arch::Arm,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Three,
};

/// `armv7s-apple-ios`: ARMv7 iOS, Cortex-a9
pub const ARMV7S_APPLE_IOS: Platform = Platform {
    target_triple: "armv7s-apple-ios",
    target_arch: Arch::Arm,
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

/// `i686-unknown-openbsd`: 32-bit OpenBSD
pub const I686_UNKNOWN_OPENBSD: Platform = Platform {
    target_triple: "i686-unknown-openbsd",
    target_arch: Arch::X86,
    target_os: OS::OpenBSD,
    target_env: None,
    tier: Tier::Three,
};

/// `mips-unknown-linux-uclibc`: MIPS Linux with uClibc
pub const MIPS_UNKNOWN_LINUX_UCLIBC: Platform = Platform {
    target_triple: "mips-unknown-linux-uclibc",
    target_arch: Arch::Mips,
    target_os: OS::Linux,
    target_env: Some(Env::UClibc),
    tier: Tier::Three,
};

/// `mipsel-unknown-linux-uclibc`: MIPS (LE) Linux with uClibc
pub const MIPSEL_UNKNOWN_LINUX_UCLIBC: Platform = Platform {
    target_triple: "mipsel-unknown-linux-uclibc",
    target_arch: Arch::Mips,
    target_os: OS::Linux,
    target_env: Some(Env::UClibc),
    tier: Tier::Three,
};

/// `msp430-none-elf`: 16-bit MSP430 microcontrollers
pub const MSP430_NONE_ELF: Platform = Platform {
    target_triple: "msp430-none-elf",
    target_arch: Arch::Msp430,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Three,
};

/// `powerpc-unknown-linux-musl`
pub const POWERPC_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "powerpc-unknown-linux-musl",
    target_arch: Arch::PowerPc,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Three,
};

/// `powerpc64-unknown-linux-musl`
pub const POWERPC64_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "powerpc64-unknown-linux-musl",
    target_arch: Arch::PowerPc64,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Three,
};

/// `powerpc64le-unknown-linux-musl`
pub const POWERPC64LE_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "powerpc64le-unknown-linux-musl",
    target_arch: Arch::PowerPc64,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Three,
};

/// `s390x-unknown-linux-musl`: S390x Linux (kernel 2.6.32, MUSL)
pub const S390X_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "s390x-unknown-linux-musl",
    target_arch: Arch::S390X,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Three,
};

/// `sparc64-unknown-netbsd`: NetBSD/sparc64
pub const SPARC64_UNKNOWN_NETBSD: Platform = Platform {
    target_triple: "sparc64-unknown-netbsd",
    target_arch: Arch::Sparc64,
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
