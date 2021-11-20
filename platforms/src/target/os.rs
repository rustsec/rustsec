//! Operating systems

use crate::error::Error;
use core::{fmt, str::FromStr};
use std::string::String;

#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};

/// `target_os`: Operating system of the target. This value is closely related to the second
/// and third element of the platform target triple, though it is not identical.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum OS {
    /// `android`: Google's Android mobile operating system
    Android,

    /// `cuda`: CUDA parallel computing platform
    Cuda,

    /// `dragonfly`: DragonflyBSD
    Dragonfly,

    /// `emscripten`: The emscripten JavaScript transpiler
    Emscripten,

    /// `freebsd`: The FreeBSD operating system
    FreeBSD,

    /// `fuchsia`: Google's next-gen Rust OS
    Fuchsia,

    /// `haiku`: Haiku, an open source BeOS clone
    Haiku,

    /// `hermit`: HermitCore is a novel unikernel operating system targeting a scalable and predictable runtime behavior for HPC and cloud environments
    Hermit,

    /// `illumos`: illumos is a partly free and open-source Unix operating system based on OpenSolaris
    Illumos,

    /// `ios`: Apple's iOS mobile operating system
    #[allow(non_camel_case_types)]
    iOS,

    /// `linux`: Linux
    Linux,

    /// `macos`: Apple's Mac OS X
    MacOS,

    /// `netbsd`: The NetBSD operating system
    NetBSD,

    /// `openbsd`: The OpenBSD operating system
    OpenBSD,

    /// `redox`: Redox, a Unix-like OS written in Rust
    Redox,

    /// `solaris`: Oracle's (formerly Sun) Solaris operating system
    Solaris,

    /// `tvOS`: AppleTV operating system
    TvOS,

    /// `wasi`: The WebAssembly System Interface
    Wasi,

    /// `windows`: Microsoft's Windows operating system
    Windows,

    /// `vxworks`: VxWorks is a deterministic, priority-based preemptive RTOS with low latency and minimal jitter.
    VxWorks,

    /// Operating systems we don't know about
    Unknown,
}

impl OS {
    /// String representing this target OS which matches `#[cfg(target_os)]`
    pub fn as_str(self) -> &'static str {
        match self {
            OS::Android => "android",
            OS::Cuda => "cuda",
            OS::Dragonfly => "dragonfly",
            OS::Emscripten => "emscripten",
            OS::FreeBSD => "freebsd",
            OS::Fuchsia => "fuchsia",
            OS::Haiku => "haiku",
            OS::Hermit => "hermit",
            OS::Illumos => "illumos",
            OS::iOS => "ios",
            OS::Linux => "linux",
            OS::MacOS => "macos",
            OS::NetBSD => "netbsd",
            OS::OpenBSD => "openbsd",
            OS::Redox => "redox",
            OS::Solaris => "solaris",
            OS::TvOS => "tvos",
            OS::Wasi => "wasi",
            OS::Windows => "windows",
            OS::VxWorks => "vxworks",
            OS::Unknown => "unknown",
        }
    }
}

impl FromStr for OS {
    type Err = Error;

    /// Create a new `Env` from the given string
    fn from_str(os_name: &str) -> Result<Self, Self::Err> {
        let os = match os_name {
            "android" => OS::Android,
            "cuda" => OS::Cuda,
            "dragonfly" => OS::Dragonfly,
            "emscripten" => OS::Emscripten,
            "freebsd" => OS::FreeBSD,
            "fuchsia" => OS::Fuchsia,
            "haiku" => OS::Haiku,
            "hermit" => OS::Hermit,
            "illumos" => OS::Illumos,
            "ios" => OS::iOS,
            "linux" => OS::Linux,
            "macos" => OS::MacOS,
            "netbsd" => OS::NetBSD,
            "openbsd" => OS::OpenBSD,
            "redox" => OS::Redox,
            "solaris" => OS::Solaris,
            "tvos" => OS::TvOS,
            "wasi" => OS::Wasi,
            "windows" => OS::Windows,
            "vxworks" => OS::VxWorks,
            _ => return Err(Error),
        };

        Ok(os)
    }
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for OS {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for OS {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(String::deserialize(deserializer)?
            .parse()
            .unwrap_or(OS::Unknown))
    }
}

// Detect and expose `target_os` as a constant
// Whether this is a good idea is somewhat debatable

#[cfg(target_os = "android")]
/// `target_os` when building this crate: `android`
pub const TARGET_OS: OS = OS::Android;

#[cfg(target_os = "cuda")]
/// `target_os` when building this crate: `cuda`
pub const TARGET_OS: OS = OS::Cuda;

#[cfg(target_os = "dragonfly")]
/// `target_os` when building this crate: `dragonfly`
pub const TARGET_OS: OS = OS::Dragonfly;

#[cfg(target_os = "emscripten")]
/// `target_os` when building this crate: `emscripten`
pub const TARGET_OS: OS = OS::Emscripten;

#[cfg(target_os = "freebsd")]
/// `target_os` when building this crate: `freebsd`
pub const TARGET_OS: OS = OS::FreeBSD;

#[cfg(target_os = "fuchsia")]
/// `target_os` when building this crate: `fuchsia`
pub const TARGET_OS: OS = OS::Fuchsia;

#[cfg(target_os = "haiku")]
/// `target_os` when building this crate: `haiku`
pub const TARGET_OS: OS = OS::Haiku;

#[cfg(target_os = "hermit")]
/// `target_os` when building this crate: `hermit`
pub const TARGET_OS: OS = OS::Hermit;

#[cfg(target_os = "illumos")]
/// `target_os` when building this crate: `illumos`
pub const TARGET_OS: OS = OS::Illumos;

#[cfg(target_os = "ios")]
/// `target_os` when building this crate: `ios`
pub const TARGET_OS: OS = OS::iOS;

#[cfg(target_os = "linux")]
/// `target_os` when building this crate: `linux`
pub const TARGET_OS: OS = OS::Linux;

#[cfg(target_os = "macos")]
/// `target_os` when building this crate: `macos`
pub const TARGET_OS: OS = OS::MacOS;

#[cfg(target_os = "netbsd")]
/// `target_os` when building this crate: `netbsd`
pub const TARGET_OS: OS = OS::NetBSD;

#[cfg(target_os = "openbsd")]
/// `target_os` when building this crate: `openbsd`
pub const TARGET_OS: OS = OS::OpenBSD;

#[cfg(target_os = "redox")]
/// `target_os` when building this crate: `redox`
pub const TARGET_OS: OS = OS::Redox;

#[cfg(target_os = "solaris")]
/// `target_os` when building this crate: `solaris`
pub const TARGET_OS: OS = OS::Solaris;

#[cfg(target_os = "tvos")]
/// `target_os` when building this crate: `tvos`
pub const TARGET_OS: OS = OS::TvOS;

#[cfg(target_os = "wasi")]
/// `target_os` when building this crate: `wasi`
pub const TARGET_OS: OS = OS::Wasi;

#[cfg(target_os = "windows")]
/// `target_os` when building this crate: `windows`
pub const TARGET_OS: OS = OS::Windows;

#[cfg(target_os = "vxworks")]
/// `target_os` when building this crate: `vxworks`
pub const TARGET_OS: OS = OS::VxWorks;

#[cfg(not(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "haiku",
    target_os = "hermit",
    target_os = "illumos",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "solaris",
    target_os = "tvos",
    target_os = "wasi",
    target_os = "windows",
    target_os = "vxworks"
)))]
/// `target_os` when building this crate: unknown!
pub const TARGET_OS: OS = OS::Unknown;


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::target::OS;

    #[test]
    fn valid_os_tests() {
        assert!(OS::from_str("linux").is_ok());
        assert!(OS::from_str("ios").is_ok());
        assert!(OS::from_str("windows").is_ok());
    }

    #[test]
    fn invalid_os_tests() {
        assert!(OS::from_str("").is_err());
        assert!(OS::from_str(" ").is_err());
        assert!(OS::from_str("derp").is_err());
        assert!(OS::from_str("***").is_err());
        assert!(OS::from_str("unknown").is_err());
    }
}
