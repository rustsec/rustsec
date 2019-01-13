//! All Tier 2 platforms supported by Rust. Sourced from:
//!
//! <https://forge.rust-lang.org/platform-support.html>
//!
//! Tier 2 platforms can be thought of as “guaranteed to build”. Automated
//! tests are not run so it’s not guaranteed to produce a working build,
//! but platforms often work to quite a good degree and patches are always
//! welcome!
//!
//! Specifically, these platforms are required to have each of the following:
//!
//! * Official binary releases are provided for the platform.
//! * Automated building is set up, but may not be running tests.
//! * Landing changes to the rust-lang/rust repository’s master branch is
//!   gated on platforms building. For some platforms only the standard
//!   library is compiled, but for others rustc and cargo are too.

use crate::{
    platform::{Platform, Tier},
    target::{Arch, Env, OS},
};

/// `aarch64-apple-ios`: ARM64 iOS
pub const AARCH64_APPLE_IOS: Platform = Platform {
    target_triple: "aarch64-apple-ios",
    target_arch: Arch::AARCH64,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Two,
};

/// `aarch64-fuchsia`: ARM64 Fuchsia
pub const AARCH64_FUCHSIA: Platform = Platform {
    target_triple: "aarch64-fuchsia",
    target_arch: Arch::AARCH64,
    target_os: OS::Fuchsia,
    target_env: None,
    tier: Tier::Two,
};

/// `aarch64-linux-android`: ARM64 Android
pub const AARCH64_LINUX_ANDROID: Platform = Platform {
    target_triple: "aarch64-linux-android",
    target_arch: Arch::AARCH64,
    target_os: OS::Android,
    target_env: None,
    tier: Tier::Two,
};

/// `aarch64-unknown-linux-gnu`: ARM64 Linux
pub const AARCH64_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "aarch64-unknown-linux-gnu",
    target_arch: Arch::AARCH64,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `aarch64-unknown-linux-musl`: ARM64 Linux with MUSL
pub const AARCH64_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "aarch64-unknown-linux-musl",
    target_arch: Arch::AARCH64,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `arm-linux-androideabi`: ARMv7 Android
pub const ARM_LINUX_ANDROIDEABI: Platform = Platform {
    target_triple: "arm-linux-androideabi",
    target_arch: Arch::ARM,
    target_os: OS::Android,
    target_env: None,
    tier: Tier::Two,
};

/// `arm-unknown-linux-gnueabi`: ARMv6 Linux
pub const ARM_UNKNOWN_LINUX_GNUEABI: Platform = Platform {
    target_triple: "arm-unknown-linux-gnueabi",
    target_arch: Arch::ARM,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `arm-unknown-linux-gnueabihf`: ARMv6 Linux, hardfloat
pub const ARM_UNKNOWN_LINUX_GNUEABIHF: Platform = Platform {
    target_triple: "arm-unknown-linux-gnueabihf",
    target_arch: Arch::ARM,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `arm-unknown-linux-musleabi`: ARMv6 Linux with MUSL
pub const ARM_UNKNOWN_LINUX_MUSLEABI: Platform = Platform {
    target_triple: "arm-unknown-linux-musleabi",
    target_arch: Arch::ARM,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `arm-unknown-linux-musleabihf`: ARMv6 Linux, MUSL, hardfloat
pub const ARM_UNKNOWN_LINUX_MUSLEABIHF: Platform = Platform {
    target_triple: "arm-unknown-linux-musleabihf",
    target_arch: Arch::ARM,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `armv5te-unknown-linux-gnueabi`: ARMv5TE Linux
pub const ARMV5TE_UNKNOWN_LINUX_GNUEABI: Platform = Platform {
    target_triple: "armv5te-unknown-linux-gnueabi",
    target_arch: Arch::ARM,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `armv7-apple-ios`: ARMv7 iOS, Cortex-a8
pub const ARMV7_APPLE_IOS: Platform = Platform {
    target_triple: "armv7-apple-ios",
    target_arch: Arch::ARM,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Two,
};

/// `armv7-linux-androideabi`: ARMv7a Android
pub const ARMV7_LINUX_ANDROIDEABI: Platform = Platform {
    target_triple: "armv7-linux-androideabi",
    target_arch: Arch::ARM,
    target_os: OS::Android,
    target_env: None,
    tier: Tier::Two,
};

/// `armv7-unknown-linux-gnueabihf`: ARMv7 Linux
pub const ARMV7_UNKNOWN_LINUX_GNUEABIHF: Platform = Platform {
    target_triple: "armv7-unknown-linux-gnueabihf",
    target_arch: Arch::ARM,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `armv7-unknown-linux-musleabihf`: ARMv7 Linux with MUSL
pub const ARMV7_UNKNOWN_LINUX_MUSLEABIHF: Platform = Platform {
    target_triple: "armv7-unknown-linux-musleabihf",
    target_arch: Arch::ARM,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `armv7s-apple-ios`: ARMv7 iOS, Cortex-a9
pub const ARMV7S_APPLE_IOS: Platform = Platform {
    target_triple: "armv7s-apple-ios",
    target_arch: Arch::ARM,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Two,
};

/// `asmjs-unknown-emscripten`: asm.js via Emscripten
pub const ASMJS_UNKNOWN_EMSCRIPTEN: Platform = Platform {
    target_triple: "asmjs-unknown-emscripten",
    target_arch: Arch::ASMJS,
    target_os: OS::Emscripten,
    target_env: None,
    tier: Tier::Two,
};

/// `i386-apple-ios`: 32-bit x86 iOS
pub const I386_APPLE_IOS: Platform = Platform {
    target_triple: "i386-apple-ios",
    target_arch: Arch::X86,
    target_os: OS::iOS,
    target_env: None,
    tier: Tier::Two,
};

/// `i586-pc-windows-msvc`: 32-bit Windows w/o SSE
pub const I586_PC_WINDOWS_MSVC: Platform = Platform {
    target_triple: "i586-pc-windows-msvc",
    target_arch: Arch::X86,
    target_os: OS::Windows,
    target_env: Some(Env::MSVC),
    tier: Tier::Two,
};

/// `i586-unknown-linux-gnu`: 32-bit Linux w/o SSE
pub const I586_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "i586-unknown-linux-gnu",
    target_arch: Arch::X86,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `i586-unknown-linux-musl`: 32-bit Linux w/o SSE, MUSL
pub const I586_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "i586-unknown-linux-musl",
    target_arch: Arch::X86,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `i686-linux-android`: 32-bit x86 Android
pub const I686_LINUX_ANDROID: Platform = Platform {
    target_triple: "i686-linux-android",
    target_arch: Arch::X86,
    target_env: None,
    target_os: OS::Android,
    tier: Tier::Two,
};

/// `i686-unknown-freebsd`: 32-bit FreeBSD
pub const I686_UNKNOWN_FREEBSD: Platform = Platform {
    target_triple: "i686-unknown-freebsd",
    target_arch: Arch::X86,
    target_os: OS::FreeBSD,
    target_env: None,
    tier: Tier::Two,
};

/// `i686-unknown-linux-musl`: 32-bit Linux with MUSL
pub const I686_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "i686-unknown-linux-musl",
    target_arch: Arch::X86,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `mips-unknown-linux-gnu`: MIPS Linux
pub const MIPS_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "mips-unknown-linux-gnu",
    target_arch: Arch::MIPS,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `mips-unknown-linux-musl`: MIPS Linux with MUSL
pub const MIPS_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "mips-unknown-linux-musl",
    target_arch: Arch::MIPS,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `mips64-unknown-linux-gnuabi64`: MIPS64 Linux, n64 ABI
pub const MIPS64_UNKNOWN_LINUX_GNUABI64: Platform = Platform {
    target_triple: "mips64-unknown-linux-gnuabi64",
    target_arch: Arch::MIPS64,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `mips64el-unknown-linux-gnuabi64`: MIPS64 (LE) Linux, n64 ABI
pub const MIPS64EL_UNKNOWN_LINUX_GNUABI64: Platform = Platform {
    target_triple: "mips64el-unknown-linux-gnuabi64",
    target_arch: Arch::MIPS64,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `mipsel-unknown-linux-gnu`: MIPS (LE) Linux
pub const MIPSEL_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "mipsel-unknown-linux-gnu",
    target_arch: Arch::MIPS,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `mipsel-unknown-linux-musl`: MIPS (LE) Linux with MUSL
pub const MIPSEL_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "mipsel-unknown-linux-musl",
    target_arch: Arch::MIPS,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `powerpc-unknown-linux-gnu`: PowerPC Linux
pub const POWERPC_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "powerpc-unknown-linux-gnu",
    target_arch: Arch::POWERPC,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `powerpc64-unknown-linux-gnu`: PPC64 Linux
pub const POWERPC64_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "powerpc64-unknown-linux-gnu",
    target_arch: Arch::POWERPC64,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `powerpc64le-unknown-linux-gnu`: PPC64LE Linux
pub const POWERPC64LE_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "powerpc64le-unknown-linux-gnu",
    target_arch: Arch::POWERPC64,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `s390x-unknown-linux-gnu`: S390x Linux
pub const S390X_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "s390x-unknown-linux-gnu",
    target_arch: Arch::S390X,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `sparc64-unknown-linux-gnu`: SPARC Linux
pub const SPARC64_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "sparc64-unknown-linux-gnu",
    target_arch: Arch::SPARC64,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `sparcv9-sun-solaris`: SPARC Solaris 10/11, illumos
pub const SPARC64_SUN_SOLARIS: Platform = Platform {
    target_triple: "sparcv9-sun-solaris",
    target_arch: Arch::SPARC64,
    target_os: OS::Solaris,
    target_env: None,
    tier: Tier::Two,
};

/// `wasm32-unknown-unknown`: WebAssembly
pub const WASM_UNKNOWN_UNKNOWN: Platform = Platform {
    target_triple: "wasm32-unknown-unknown",
    target_arch: Arch::WASM32,
    target_os: OS::Unknown,
    target_env: None,
    tier: Tier::Two,
};

/// `wasm32-unknown-emscripten`: WebAssembly via Emscripten
pub const WASM_UNKNOWN_EMSCRIPTEN: Platform = Platform {
    target_triple: "wasm32-unknown-emscripten",
    target_arch: Arch::WASM32,
    target_os: OS::Emscripten,
    target_env: None,
    tier: Tier::Two,
};

/// `x86_64-apple-ios`: 64-bit x86 iOS
pub const X86_64_APPLE_IOS: Platform = Platform {
    target_triple: "x86_64-apple-ios",
    target_arch: Arch::X86_64,
    target_env: None,
    target_os: OS::iOS,
    tier: Tier::Two,
};

/// `x86_64-fuchsia`: 64-bit x86 Fuchsia
pub const X86_64_FUCHSIA: Platform = Platform {
    target_triple: "x86_64-fuchsia",
    target_arch: Arch::X86_64,
    target_os: OS::Fuchsia,
    target_env: None,
    tier: Tier::Two,
};

/// `x86_64-linux-android`: 64-bit x86 Android
pub const X86_64_LINUX_ANDROID: Platform = Platform {
    target_triple: "x86_64-linux-android",
    target_arch: Arch::X86_64,
    target_env: None,
    target_os: OS::Android,
    tier: Tier::Two,
};

/// `x86_64-rumprun-netbsd`: 64-bit NetBSD Rump Kernel
pub const X86_64_RUMPRUN_NETBSD: Platform = Platform {
    target_triple: "x86_64-rumprun-netbsd",
    target_arch: Arch::X86_64,
    target_env: None,
    target_os: OS::NetBSD,
    tier: Tier::Two,
};

/// `x86_64-sun-solaris`: 64-bit Solaris 10/11, illumos
pub const X86_64_SUN_SOLARIS: Platform = Platform {
    target_triple: "x86_64-sun-solaris",
    target_arch: Arch::X86_64,
    target_os: OS::Solaris,
    target_env: None,
    tier: Tier::Two,
};

/// `x86_64-unknown-cloudabi`: 64-bit CloudABI
pub const X86_64_UNKNOWN_CLOUDABI: Platform = Platform {
    target_triple: "x86_64-unknown-cloudabi",
    target_arch: Arch::X86_64,
    target_env: None,
    target_os: OS::CloudABI,
    tier: Tier::Two,
};

/// `x86_64-unknown-freebsd`: 64-bit FreeBSD
pub const X86_64_UNKNOWN_FREEBSD: Platform = Platform {
    target_triple: "x86_64-unknown-freebsd",
    target_arch: Arch::X86_64,
    target_os: OS::FreeBSD,
    target_env: None,
    tier: Tier::Two,
};

/// `x86_64-unknown-linux-gnux32`: 64-bit Linux
pub const X86_64_UNKNOWN_LINUX_GNUX32: Platform = Platform {
    target_triple: "x86_64-unknown-linux-gnux32",
    target_arch: Arch::X86_64,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `x86_64-unknown-linux-musl`: 64-bit Linux with MUSL
pub const X86_64_UNKNOWN_LINUX_MUSL: Platform = Platform {
    target_triple: "x86_64-unknown-linux-musl",
    target_arch: Arch::X86_64,
    target_os: OS::Linux,
    target_env: Some(Env::Musl),
    tier: Tier::Two,
};

/// `x86_64-unknown-netbsd`: NetBSD/amd64
pub const X86_64_UNKNOWN_NETBSD: Platform = Platform {
    target_triple: "x86_64-unknown-netbsd",
    target_arch: Arch::X86_64,
    target_env: None,
    target_os: OS::NetBSD,
    tier: Tier::Two,
};

/// `x86_64-unknown-redox`: Redox OS
pub const X86_64_UNKNOWN_REDOX: Platform = Platform {
    target_triple: "x86_64-unknown-redox",
    target_arch: Arch::X86_64,
    target_env: None,
    target_os: OS::Redox,
    tier: Tier::Two,
};

//
// Tier 2.5 platforms
//
// Tier 2.5 platforms can be thought of as “guaranteed to build”, but without
// builds available through rustup. Automated tests are not run so it’s not
// guaranteed to produce a working build, but platforms often work to quite a
// good degree and patches are always welcome! Specifically, these platforms
// are required to have each of the following:
//
// - Automated building is set up, but may not be running tests.
// - Landing changes to the rust-lang/rust repository’s master branch is gated
//   on platforms building. For some platforms only the standard library is
//   compiled, but for others rustc and cargo are too.
//
// **This status is accidental: no new platforms should reach this state**
//

/// `aarch64-unknown-cloudabi`: ARM64 CloudABI
pub const AARCH64_UNKNOWN_CLOUDABI: Platform = Platform {
    target_triple: "aarch64-unknown-cloudabi",
    target_arch: Arch::AARCH64,
    target_os: OS::CloudABI,
    target_env: None,
    tier: Tier::Two,
};

/// `armv7-unknown-cloudabi-eabihf`: ARMv7 CloudABI, hardfloat
pub const ARMV7_UNKNOWN_CLOUDABI_EABIHF: Platform = Platform {
    target_triple: "armv7-unknown-cloudabi-eabihf",
    target_arch: Arch::ARM,
    target_os: OS::CloudABI,
    target_env: None,
    tier: Tier::Two,
};

/// `i686-unknown-cloudabi`: 32-bit CloudABI
pub const I686_UNKNOWN_CLOUDABI: Platform = Platform {
    target_triple: "i686-unknown-cloudabi",
    target_arch: Arch::X86,
    target_os: OS::CloudABI,
    target_env: None,
    tier: Tier::Two,
};

/// `powerpc-unknown-linux-gnuspe`: PowerPC SPE Linux
pub const POWERPC_UNKNOWN_LINUX_GNUSPE: Platform = Platform {
    target_triple: "powerpc-unknown-linux-gnuspe",
    target_arch: Arch::POWERPC,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};

/// `sparc-unknown-linux-gnu`: 32-bit SPARC Linux
pub const SPARC_UNKNOWN_LINUX_GNU: Platform = Platform {
    target_triple: "sparc-unknown-linux-gnu",
    target_arch: Arch::SPARC,
    target_os: OS::Linux,
    target_env: Some(Env::GNU),
    tier: Tier::Two,
};
