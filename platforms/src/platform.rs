//! Rust platforms

mod tier1;
mod tier2;
mod tier3;

#[cfg(feature = "std")]
mod req;
mod tier;

pub use self::{tier::Tier, tier1::*, tier2::*, tier3::*};

#[cfg(feature = "std")]
pub use self::req::PlatformReq;

use crate::target::*;
use core::fmt;

/// Rust platforms supported by mainline rustc
///
/// Sourced from <https://doc.rust-lang.org/nightly/rustc/platform-support.html>
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Platform {
    /// "Target triple" string uniquely identifying the platform. See:
    /// <https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md>
    ///
    /// These are defined in the `rustc_target` crate of the Rust compiler:
    /// <https://github.com/rust-lang/rust/blob/master/src/librustc_target/spec/mod.rs>
    pub target_triple: &'static str,

    /// Target architecture `cfg` attribute (i.e. `cfg(target_arch)`)
    pub target_arch: Arch,

    /// Target OS `cfg` attribute (i.e. `cfg(target_os)`).
    pub target_os: OS,

    /// Target environment `cfg` attribute (i.e. `cfg(target_env)`).
    /// Only used when needed for disambiguation, e.g. on many GNU platforms
    /// this value will be `None`.
    pub target_env: Option<Env>,

    /// Tier of this platform:
    ///
    /// - `Tier::One`: guaranteed to work
    /// - `Tier::Two`: guaranteed to build
    /// - `Tier::Three`: unofficially supported with no guarantees
    pub tier: Tier,
}

impl Platform {
    /// All valid Rust platforms usable from the mainline compiler
    pub const ALL: &'static [Platform] = &[
        // Tier 1
        AARCH64_UNKNOWN_LINUX_GNU,
        I686_PC_WINDOWS_GNU,
        I686_PC_WINDOWS_MSVC,
        I686_UNKNOWN_LINUX_GNU,
        X86_64_APPLE_DARWIN,
        X86_64_PC_WINDOWS_GNU,
        X86_64_PC_WINDOWS_MSVC,
        X86_64_UNKNOWN_LINUX_GNU,
        // Tier 2
        AARCH64_APPLE_DARWIN,
        AARCH64_APPLE_IOS,
        AARCH64_PC_WINDOWS_MSVC,
        AARCH64_LINUX_ANDROID,
        AARCH64_FUCHSIA,
        AARCH64_UNKNOWN_LINUX_MUSL,
        AARCH64_UNKNOWN_NONE,
        AARCH64_UNKNOWN_NONE_SOFTFLOAT,
        ARM_LINUX_ANDROIDEABI,
        ARM_UNKNOWN_LINUX_GNUEABI,
        ARM_UNKNOWN_LINUX_GNUEABIHF,
        ARM_UNKNOWN_LINUX_MUSLEABI,
        ARM_UNKNOWN_LINUX_MUSLEABIHF,
        ARMV5TE_UNKNOWN_LINUX_GNUEABI,
        ARMV5TE_UNKNOWN_LINUX_MUSLEABI,
        ARMV7_LINUX_ANDROIDEABI,
        ARMV7_UNKNOWN_LINUX_GNUEABI,
        ARMV7_UNKNOWN_LINUX_GNUEABIHF,
        ARMV7_UNKNOWN_LINUX_MUSLEABI,
        ARMV7_UNKNOWN_LINUX_MUSLEABIHF,
        ARMEBV7R_NONE_EABI,
        ARMEBV7R_NONE_EABIHF,
        ASMJS_UNKNOWN_EMSCRIPTEN,
        I586_PC_WINDOWS_MSVC,
        I586_UNKNOWN_LINUX_GNU,
        I586_UNKNOWN_LINUX_MUSL,
        I686_LINUX_ANDROID,
        I686_UNKNOWN_FREEBSD,
        I686_UNKNOWN_LINUX_MUSL,
        MIPS_UNKNOWN_LINUX_GNU,
        MIPS_UNKNOWN_LINUX_MUSL,
        MIPS64_UNKNOWN_LINUX_GNUABI64,
        MIPS64_UNKNOWN_LINUX_MUSLABI64,
        MIPS64EL_UNKNOWN_LINUX_GNUABI64,
        MIPS64EL_UNKNOWN_LINUX_MUSLABI64,
        MIPSEL_UNKNOWN_LINUX_GNU,
        MIPSEL_UNKNOWN_LINUX_MUSL,
        NVPTX64_NVIDIA_CUDA,
        POWERPC_UNKNOWN_LINUX_GNU,
        POWERPC64_UNKNOWN_LINUX_GNU,
        POWERPC64LE_UNKNOWN_LINUX_GNU,
        S390X_UNKNOWN_LINUX_GNU,
        SPARC64_UNKNOWN_LINUX_GNU,
        SPARC64_SUN_SOLARIS,
        THUMBV6M_NONE_EABI,
        THUMBV7EM_NONE_EABI,
        THUMBV7EM_NONE_EABIHF,
        THUMBV7M_NONE_EABI,
        THUMBV7NEON_LINUX_ANDROIDEABI,
        THUMBV7NEON_UNKNOWN_LINUX_GNUEABIHF,
        WASM_UNKNOWN_UNKNOWN,
        WASM_UNKNOWN_EMSCRIPTEN,
        WASM_WASI,
        X86_64_APPLE_IOS,
        X86_64_FORTANIX_UNKNOWN_SGX,
        X86_64_LINUX_ANDROID,
        X86_64_PC_SOLARIS,
        X86_64_UNKNOWN_FREEBSD,
        X86_64_FUCHSIA,
        X86_64_UNKNOWN_ILLUMOS,
        X86_64_UNKNOWN_LINUX_GNUX32,
        X86_64_UNKNOWN_LINUX_MUSL,
        X86_64_UNKNOWN_NETBSD,
        X86_64_UNKNOWN_REDOX,
        // Tier 2.5
        POWERPC_UNKNOWN_LINUX_GNUSPE,
        SPARC_UNKNOWN_LINUX_GNU,
        // Tier 3
        AARCH64_APPLE_IOS_MACABI,
        AARCH64_APPLE_IOS_SIM,
        AARCH64_APPLE_TVOS,
        AARCH64_UNKNOWN_FREEBSD,
        AARCH64_UNKNOWN_HERMIT,
        AARCH64_UNKNOWN_LINUX_GNU_ILP32,
        AARCH64_UNKNOWN_NETBSD,
        AARCH64_UNKNOWN_OPENBSD,
        AARCH64_UNKNOWN_REDOX,
        AARCH64_UWP_WINDOWS_MSVC,
        AARCH64_WRS_VXWORKS,
        AARCH64_BE_UNKNOWN_LINUX_GNU_ILP32,
        AARCH64_BE_UNKNOWN_LINUX_GNU,
        ARMV4T_UNKNOWN_LINUX_GNUEABI,
        ARMV5T_UNKNOWN_LINUX_UCLIBCEABI,
        ARMV6_UNKNOWN_FREEBSD,
        ARMV6_UNKNOWN_NETBSD_EABIHF,
        ARMV7_APPLE_IOS,
        ARMV7_UNKNOWN_FREEBSD,
        ARMV7_UNKNOWN_NETBSD_EABIHF,
        ARMV7_WRS_VXWORKS_EABIHF,
        ARMV7A_NONE_EABIHF,
        ARMV7S_APPLE_IOS,
        I386_APPLE_IOS,
        I686_APPLE_DARWIN,
        I686_UNKNOWN_HAIKU,
        I686_UNKNOWN_NETBSD,
        I686_UNKNOWN_OPENBSD,
        MIPS_UNKNOWN_LINUX_UCLIBC,
        MIPSEL_UNKNOWN_LINUX_UCLIBC,
        MSP430_NONE_ELF,
        POWERPC_UNKNOWN_LINUX_MUSL,
        POWERPC64_UNKNOWN_LINUX_MUSL,
        POWERPC64LE_UNKNOWN_LINUX_MUSL,
        S390X_UNKNOWN_LINUX_MUSL,
        SPARC64_UNKNOWN_NETBSD,
        X86_64_SUN_SOLARIS,
        X86_64_UNKNOWN_DRAGONFLY,
        X86_64_UNKNOWN_HAIKU,
        X86_64_UNKNOWN_OPENBSD,
    ];

    /// Find a Rust platform by its "target triple", e.g. `i686-apple-darwin`
    pub fn find(target_triple: &str) -> Option<&'static Platform> {
        Self::ALL
            .iter()
            .find(|platform| platform.target_triple == target_triple)
    }

    /// Attempt to guess the current `Platform`. May give inaccurate results.
    pub fn guess_current() -> Option<&'static Platform> {
        Self::find(env!("TARGET")).or_else(|| {
            Self::ALL.iter().find(|&platform| {
                platform.target_arch == TARGET_ARCH
                    && platform.target_env == TARGET_ENV
                    && platform.target_os == TARGET_OS
            })
        })
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.target_triple)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::Platform;
    use std::collections::HashSet;

    /// Ensure there are no duplicate target triples in the platforms list
    #[test]
    fn no_dupes_test() {
        let mut target_triples = HashSet::new();

        for platform in Platform::ALL {
            assert!(
                target_triples.insert(platform.target_triple),
                "duplicate target triple: {}",
                platform.target_triple
            );
        }
    }

    #[test]
    fn guesses_current() {
        assert!(Platform::guess_current().is_some());
    }
}
