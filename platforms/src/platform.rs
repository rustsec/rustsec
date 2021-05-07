//! Rust platforms

pub mod tier1;
pub mod tier2;
pub mod tier3;

#[cfg(feature = "std")]
mod req;
mod tier;

pub use self::tier::Tier;

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
    /// Find a Rust platform by its "target triple", e.g. `i686-apple-darwin`
    pub fn find(target_triple: &str) -> Option<&'static Platform> {
        Self::all()
            .iter()
            .find(|platform| platform.target_triple == target_triple)
    }

    /// Attempt to guess the current `Platform`. May give inaccurate results.
    pub fn guess_current() -> Option<&'static Platform> {
        Self::find(env!("TARGET")).or_else(|| {
            Self::all().iter().find(|platform| {
                platform.target_arch == TARGET_ARCH
                    && platform.target_env == TARGET_ENV
                    && platform.target_os == TARGET_OS
            })
        })
    }

    /// All valid Rust platforms usable from the mainline compiler
    pub fn all() -> &'static [Platform] {
        &[
            // Tier 1
            tier1::I686_APPLE_DARWIN,
            tier1::I686_PC_WINDOWS_GNU,
            tier1::I686_PC_WINDOWS_MSVC,
            tier1::I686_UNKNOWN_LINUX_GNU,
            tier1::X86_64_APPLE_DARWIN,
            tier1::X86_64_PC_WINDOWS_GNU,
            tier1::X86_64_PC_WINDOWS_MSVC,
            tier1::X86_64_UNKNOWN_LINUX_GNU,
            // Tier 2
            tier2::AARCH64_APPLE_DARWIN,
            tier2::AARCH64_APPLE_IOS,
            tier2::AARCH64_PC_WINDOWS_MSVC,
            tier2::AARCH64_LINUX_ANDROID,
            tier2::AARCH64_FUCHSIA,
            tier2::AARCH64_UNKNOWN_LINUX_GNU,
            tier2::AARCH64_UNKNOWN_LINUX_MUSL,
            tier2::ARM_LINUX_ANDROIDEABI,
            tier2::ARM_UNKNOWN_LINUX_GNUEABI,
            tier2::ARM_UNKNOWN_LINUX_GNUEABIHF,
            tier2::ARM_UNKNOWN_LINUX_MUSLEABI,
            tier2::ARM_UNKNOWN_LINUX_MUSLEABIHF,
            tier2::ARMV5TE_UNKNOWN_LINUX_GNUEABI,
            tier2::ARMV7_APPLE_IOS,
            tier2::ARMV7_LINUX_ANDROIDEABI,
            tier2::ARMV7_UNKNOWN_LINUX_GNUEABIHF,
            tier2::ARMV7_UNKNOWN_LINUX_MUSLEABIHF,
            tier2::ARMV7S_APPLE_IOS,
            tier2::ASMJS_UNKNOWN_EMSCRIPTEN,
            tier2::I386_APPLE_IOS,
            tier2::I586_PC_WINDOWS_MSVC,
            tier2::I586_UNKNOWN_LINUX_GNU,
            tier2::I586_UNKNOWN_LINUX_MUSL,
            tier2::I686_LINUX_ANDROID,
            tier2::I686_UNKNOWN_FREEBSD,
            tier2::I686_UNKNOWN_LINUX_MUSL,
            tier2::MIPS_UNKNOWN_LINUX_GNU,
            tier2::MIPS_UNKNOWN_LINUX_MUSL,
            tier2::MIPS64_UNKNOWN_LINUX_GNUABI64,
            tier2::MIPS64EL_UNKNOWN_LINUX_GNUABI64,
            tier2::MIPSEL_UNKNOWN_LINUX_GNU,
            tier2::MIPSEL_UNKNOWN_LINUX_MUSL,
            tier2::POWERPC_UNKNOWN_LINUX_GNU,
            tier2::POWERPC64_UNKNOWN_LINUX_GNU,
            tier2::POWERPC64LE_UNKNOWN_LINUX_GNU,
            tier2::S390X_UNKNOWN_LINUX_GNU,
            tier2::SPARC64_UNKNOWN_LINUX_GNU,
            tier2::SPARC64_SUN_SOLARIS,
            tier2::WASM_UNKNOWN_UNKNOWN,
            tier2::WASM_UNKNOWN_EMSCRIPTEN,
            tier2::X86_64_APPLE_IOS,
            tier2::X86_64_LINUX_ANDROID,
            tier2::X86_64_RUMPRUN_NETBSD,
            tier2::X86_64_SUN_SOLARIS,
            tier2::X86_64_UNKNOWN_CLOUDABI,
            tier2::X86_64_UNKNOWN_FREEBSD,
            tier2::X86_64_FUCHSIA,
            tier2::X86_64_UNKNOWN_LINUX_GNUX32,
            tier2::X86_64_UNKNOWN_LINUX_MUSL,
            tier2::X86_64_UNKNOWN_NETBSD,
            tier2::X86_64_UNKNOWN_REDOX,
            // Tier 2.5
            tier2::AARCH64_UNKNOWN_CLOUDABI,
            tier2::ARMV7_UNKNOWN_CLOUDABI_EABIHF,
            tier2::I686_UNKNOWN_CLOUDABI,
            tier2::POWERPC_UNKNOWN_LINUX_GNUSPE,
            tier2::SPARC_UNKNOWN_LINUX_GNU,
            // Tier 3
            tier3::I686_UNKNOWN_HAIKU,
            tier3::I686_UNKNOWN_NETBSD,
            tier3::MIPS_UNKNOWN_LINUX_UCLIBC,
            tier3::MIPSEL_UNKNOWN_LINUX_UCLIBC,
            tier3::MSP430_NONE_ELF,
            tier3::SPARC64_UNKNOWN_NETBSD,
            tier3::THUMBV6M_NONE_EABI,
            tier3::THUMBV7EM_NONE_EABI,
            tier3::THUMBV7EM_NONE_EABIHF,
            tier3::THUMBV7M_NONE_EABI,
            tier3::X86_64_FORTANIX_UNKNOWN_SGX,
            tier3::X86_64_UNKNOWN_BITRIG,
            tier3::X86_64_UNKNOWN_DRAGONFLY,
            tier3::X86_64_UNKNOWN_HAIKU,
            tier3::X86_64_UNKNOWN_OPENBSD,
        ]
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

        for platform in Platform::all() {
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
