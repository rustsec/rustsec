use core::str::FromStr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;

/// `target_os`: Operating system of the target. This value is closely related to the second
/// and third element of the platform target triple, though it is not identical.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum OS {
    /// `android`: Google's Android mobile operating system
    Android,

    /// `bitrig`: OpenBSD-based operating system
    Bitrig,

    /// `cloudabi`: Nuxi CloudABI runtime environment
    CloudABI,

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

    /// `windows`: Microsoft's Windows operating system
    Windows,

    /// Operating systems we don't know about
    Unknown,
}

impl OS {
    /// String representing this target OS which matches `#[cfg(target_os)]`
    pub fn as_str(self) -> &'static str {
        match self {
            OS::Android => "android",
            OS::Bitrig => "bitrig",
            OS::CloudABI => "cloudabi",
            OS::Dragonfly => "dragonfly",
            OS::Emscripten => "emscripten",
            OS::FreeBSD => "freebsd",
            OS::Fuchsia => "fuchsia",
            OS::Haiku => "haiku",
            OS::iOS => "ios",
            OS::Linux => "linux",
            OS::MacOS => "macos",
            OS::NetBSD => "netbsd",
            OS::OpenBSD => "openbsd",
            OS::Redox => "redox",
            OS::Solaris => "solaris",
            OS::Windows => "windows",
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
            "bitrig" => OS::Bitrig,
            "cloudabi" => OS::CloudABI,
            "dragonfly" => OS::Dragonfly,
            "emscripten" => OS::Emscripten,
            "freebsd" => OS::FreeBSD,
            "fuchsia" => OS::Fuchsia,
            "haiku" => OS::Haiku,
            "ios" => OS::iOS,
            "linux" => OS::Linux,
            "macos" => OS::MacOS,
            "netbsd" => OS::NetBSD,
            "openbsd" => OS::OpenBSD,
            "redox" => OS::Redox,
            "solaris" => OS::Solaris,
            "windows" => OS::Windows,
            _ => return Err(Error),
        };

        Ok(os)
    }
}

#[cfg(feature = "serde")]
impl Serialize for OS {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for OS {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_str(<&str>::deserialize(deserializer)?).unwrap_or(OS::Unknown))
    }
}

// Detect and expose `target_os` as a constant
// Whether this is a good idea is somewhat debatable

#[cfg(target_os = "android")]
/// `target_os` when building this crate: `android`
pub const TARGET_OS: OS = OS::Android;

#[cfg(target_os = "bitrig")]
/// `target_os` when building this crate: `bitrig`
pub const TARGET_OS: OS = OS::Bitrig;

#[cfg(target_os = "cloudabi")]
/// `target_os` when building this crate: `cloudabi`
pub const TARGET_OS: OS = OS::CloudABI;

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

#[cfg(target_os = "windows")]
/// `target_os` when building this crate: `windows`
pub const TARGET_OS: OS = OS::Windows;

#[cfg(not(any(
    target_os = "android",
    target_os = "bitrig",
    target_os = "cloudabi",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "haiku",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "solaris",
    target_os = "windows",
)))]
/// `target_os` when building this crate: unknown!
pub const TARGET_OS: OS = OS::Unknown;
