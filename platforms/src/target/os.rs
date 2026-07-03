//! Operating systems

use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, de::Error as DeError, ser, Deserialize, Serialize};

use crate::error::Error;

/// `target_os`: Operating system of the target.
///
/// This value is closely related to the second and third element
/// of the platform target triple, though it is not identical.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Os {
    /// `aix`
    Aix,

    /// `amdhsa`
    Amdhsa,

    /// `android`: Google's Android mobile operating system
    Android,

    /// `cuda`: CUDA parallel computing platform
    Cuda,

    /// `cygwin`
    Cygwin,

    /// `dragonfly`: DragonflyBSD
    Dragonfly,

    /// `emscripten`: The emscripten JavaScript transpiler
    Emscripten,

    /// `espidf`
    Espidf,

    /// `freebsd`: The FreeBSD operating system
    FreeBSD,

    /// `fuchsia`: Google's next-gen Rust OS
    Fuchsia,

    /// `haiku`: Haiku, an open source BeOS clone
    Haiku,

    /// `helenos`
    HelenOS,

    /// `hermit`: HermitCore is a novel unikernel operating system targeting a scalable and predictable runtime behavior for HPC and cloud environments
    Hermit,

    /// `horizon`
    Horizon,

    /// `hurd`
    Hurd,

    /// `illumos`: illumos is a partly free and open-source Unix operating system based on OpenSolaris
    IllumOS,

    /// `ios`: Apple's iOS mobile operating system
    #[allow(non_camel_case_types)]
    iOS,

    /// `l4re`
    L4re,

    /// `linux`: Linux
    Linux,

    /// `lynxos178`
    Lynxos178,

    /// `macos`: Apple's Mac OS X
    MacOS,

    /// `managarm`
    Managarm,

    /// `motor`
    Motor,

    /// `netbsd`: The NetBSD operating system
    NetBSD,

    /// `none`
    None,

    /// `nto`
    Nto,

    /// `nuttx`
    Nuttx,

    /// `openbsd`: The OpenBSD operating system
    OpenBSD,

    /// `psp`
    Psp,

    /// `psx`
    Psx,

    /// `qnx`
    Qnx,

    /// `qurt`
    Qurt,

    /// `redox`: Redox, a Unix-like OS written in Rust
    Redox,

    /// `rtems`
    Rtems,

    /// `solaris`: Oracle's (formerly Sun) Solaris operating system
    Solaris,

    /// `solid_asp3`
    SolidAsp3,

    /// `teeos`
    TeeOS,

    /// `trusty`
    Trusty,

    /// `tvos`
    TvOS,

    /// `uefi`
    Uefi,

    /// `unknown`
    Unknown,

    /// `vexos`
    VexOS,

    /// `visionos`
    VisionOS,

    /// `vita`
    Vita,

    /// `vxworks`: VxWorks is a deterministic, priority-based preemptive RTOS with low latency and minimal jitter
    VxWorks,

    /// `wasi`: The WebAssembly System Interface
    Wasi,

    /// `watchos`
    WatchOS,

    /// `windows`: Microsoft's Windows operating system
    Windows,

    /// `xous`
    Xous,

    /// `zkvm`
    Zkvm,
}

impl Os {
    /// String representing this `Os` which matches `#[cfg(target_os)]`
    pub fn as_str(self) -> &'static str {
        match self {
            Os::Aix => "aix",
            Os::Amdhsa => "amdhsa",
            Os::Android => "android",
            Os::Cuda => "cuda",
            Os::Cygwin => "cygwin",
            Os::Dragonfly => "dragonfly",
            Os::Emscripten => "emscripten",
            Os::Espidf => "espidf",
            Os::FreeBSD => "freebsd",
            Os::Fuchsia => "fuchsia",
            Os::Haiku => "haiku",
            Os::HelenOS => "helenos",
            Os::Hermit => "hermit",
            Os::Horizon => "horizon",
            Os::Hurd => "hurd",
            Os::IllumOS => "illumos",
            Os::iOS => "ios",
            Os::L4re => "l4re",
            Os::Linux => "linux",
            Os::Lynxos178 => "lynxos178",
            Os::MacOS => "macos",
            Os::Managarm => "managarm",
            Os::Motor => "motor",
            Os::NetBSD => "netbsd",
            Os::None => "none",
            Os::Nto => "nto",
            Os::Nuttx => "nuttx",
            Os::OpenBSD => "openbsd",
            Os::Psp => "psp",
            Os::Psx => "psx",
            Os::Qnx => "qnx",
            Os::Qurt => "qurt",
            Os::Redox => "redox",
            Os::Rtems => "rtems",
            Os::Solaris => "solaris",
            Os::SolidAsp3 => "solid_asp3",
            Os::TeeOS => "teeos",
            Os::Trusty => "trusty",
            Os::TvOS => "tvos",
            Os::Uefi => "uefi",
            Os::Unknown => "unknown",
            Os::VexOS => "vexos",
            Os::VisionOS => "visionos",
            Os::Vita => "vita",
            Os::VxWorks => "vxworks",
            Os::Wasi => "wasi",
            Os::WatchOS => "watchos",
            Os::Windows => "windows",
            Os::Xous => "xous",
            Os::Zkvm => "zkvm",
        }
    }
}

impl FromStr for Os {
    type Err = Error;

    /// Create a new `Os` from the given string
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let result = match name {
            "aix" => Os::Aix,
            "amdhsa" => Os::Amdhsa,
            "android" => Os::Android,
            "cuda" => Os::Cuda,
            "cygwin" => Os::Cygwin,
            "dragonfly" => Os::Dragonfly,
            "emscripten" => Os::Emscripten,
            "espidf" => Os::Espidf,
            "freebsd" => Os::FreeBSD,
            "fuchsia" => Os::Fuchsia,
            "haiku" => Os::Haiku,
            "helenos" => Os::HelenOS,
            "hermit" => Os::Hermit,
            "horizon" => Os::Horizon,
            "hurd" => Os::Hurd,
            "illumos" => Os::IllumOS,
            "ios" => Os::iOS,
            "l4re" => Os::L4re,
            "linux" => Os::Linux,
            "lynxos178" => Os::Lynxos178,
            "macos" => Os::MacOS,
            "managarm" => Os::Managarm,
            "motor" => Os::Motor,
            "netbsd" => Os::NetBSD,
            "none" => Os::None,
            "nto" => Os::Nto,
            "nuttx" => Os::Nuttx,
            "openbsd" => Os::OpenBSD,
            "psp" => Os::Psp,
            "psx" => Os::Psx,
            "qnx" => Os::Qnx,
            "qurt" => Os::Qurt,
            "redox" => Os::Redox,
            "rtems" => Os::Rtems,
            "solaris" => Os::Solaris,
            "solid_asp3" => Os::SolidAsp3,
            "teeos" => Os::TeeOS,
            "trusty" => Os::Trusty,
            "tvos" => Os::TvOS,
            "uefi" => Os::Uefi,
            "unknown" => Os::Unknown,
            "vexos" => Os::VexOS,
            "visionos" => Os::VisionOS,
            "vita" => Os::Vita,
            "vxworks" => Os::VxWorks,
            "wasi" => Os::Wasi,
            "watchos" => Os::WatchOS,
            "windows" => Os::Windows,
            "xous" => Os::Xous,
            "zkvm" => Os::Zkvm,
            _ => return Err(Error),
        };

        Ok(result)
    }
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Os {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(all(feature = "serde", feature = "std"))]
impl<'de> Deserialize<'de> for Os {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = std::string::String::deserialize(deserializer)?;
        string.parse().map_err(|_| {
            D::Error::custom(std::format!(
                "Unrecognized value '{}' for target_os",
                string
            ))
        })
    }
}
