//! Rust architectures

use crate::error::Error;
use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};

/// `target_arch`: Target CPU architecture
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Arch {
    /// `aarch64`: ARMv8 64-bit architecture
    AArch64,

    /// `arm`: 32-bit ARM architecture
    Arm,

    /// `asm`: asm.js output
    AsmJs,

    /// `mips`: 32-bit MIPS CPU architecture
    Mips,

    /// `mips64`: 64-bit MIPS CPU architecture
    Mips64,

    /// `msp430`: 16-bit MSP430 microcontrollers
    Msp430,

    /// `nvptx64`: 64-bit NVIDIA PTX
    Nvptx64,

    /// `powerpc`: 32-bit POWERPC platform
    PowerPc,

    /// `powerpc64`: 64-bit POWERPC platform
    PowerPc64,

    /// `riscv`: RISC-V CPU architecture
    RiscV,

    /// `s390x`: 64-bit IBM z/Architecture
    S390X,

    /// `sparc`: 32-bit SPARC CPU architecture
    Sparc,

    /// `sparc64`: 64-bit SPARC CPU architecture
    Sparc64,

    /// `thumbv6`: 16-bit ARM CPU architecture subset
    ThumbV6,

    /// `thumbv7`: 16-bit ARM CPU architecture subset
    ThumbV7,

    /// `wasm32`: Web Assembly (32-bit)
    Wasm32,

    /// `x86`: Generic x86 CPU architecture
    X86,

    /// `x86_64`: "AMD64" CPU architecture
    X86_64,

    /// Unknown CPU architecture
    Unknown,
}

impl Arch {
    /// String representing this target architecture which matches `cfg(target_arch)`
    pub fn as_str(self) -> &'static str {
        match self {
            Arch::AArch64 => "aarch64",
            Arch::Arm => "arm",
            Arch::AsmJs => "asmjs",
            Arch::Mips => "mips",
            Arch::Mips64 => "mips64",
            Arch::Msp430 => "msp430",
            Arch::Nvptx64 => "nvptx64",
            Arch::PowerPc => "powerpc",
            Arch::PowerPc64 => "powerpc64",
            Arch::RiscV => "riscv",
            Arch::S390X => "s390x",
            Arch::Sparc => "sparc",
            Arch::Sparc64 => "sparc64",
            Arch::ThumbV6 => "thumbv6",
            Arch::ThumbV7 => "thumbv7",
            Arch::Wasm32 => "wasm32",
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::Unknown => "unknown",
        }
    }
}

impl FromStr for Arch {
    type Err = Error;

    /// Create a new `Arch` from the given string
    fn from_str(arch_name: &str) -> Result<Self, Self::Err> {
        let arch = match arch_name {
            "aarch64" => Arch::AArch64,
            "arm" => Arch::Arm,
            "asmjs" => Arch::AsmJs,
            "mips" => Arch::Mips,
            "mips64" => Arch::Mips64,
            "msp430" => Arch::Msp430,
            "nvptx64" => Arch::Nvptx64,
            "powerpc" => Arch::PowerPc,
            "powerpc64" => Arch::PowerPc64,
            "riscv" => Arch::RiscV,
            "s390x" => Arch::S390X,
            "sparc" => Arch::Sparc,
            "sparc64" => Arch::Sparc64,
            "thumbv6" => Arch::ThumbV6,
            "thumbv7" => Arch::ThumbV7,
            "wasm32" => Arch::Wasm32,
            "x86" => Arch::X86,
            "x86_64" => Arch::X86_64,
            _ => return Err(Error),
        };

        Ok(arch)
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Arch {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Arch {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(<&str>::deserialize(deserializer)?
            .parse()
            .unwrap_or(Arch::Unknown))
    }
}

// Detect and expose `target_arch` as a constant
// Whether this is a good idea is somewhat debatable

#[cfg(target_arch = "aarch64")]
/// `target_arch` when building this crate: `x86_64`
pub const TARGET_ARCH: Arch = Arch::AArch64;

#[cfg(target_arch = "arm")]
/// `target_arch` when building this crate: `arm`
pub const TARGET_ARCH: Arch = Arch::Arm;

#[cfg(target_arch = "asmjs")]
/// `target_arch` when building this crate: `asmjs`
pub const TARGET_ARCH: Arch = Arch::AsmJs;

#[cfg(target_arch = "mips")]
/// `target_arch` when building this crate: `mips`
pub const TARGET_ARCH: Arch = Arch::Mips;

#[cfg(target_arch = "mips64")]
/// `target_arch` when building this crate: `mips64`
pub const TARGET_ARCH: Arch = Arch::Mips64;

#[cfg(target_arch = "msp430")]
/// `target_arch` when building this crate: `msp430`
pub const TARGET_ARCH: Arch = Arch::Msp430;

#[cfg(target_arch = "nvptx64")]
/// `target_arch` when building this crate: `nvptx64`
pub const TARGET_ARCH: Arch = Arch::Nvptx64;

#[cfg(target_arch = "powerpc")]
/// `target_arch` when building this crate: `powerpc`
pub const TARGET_ARCH: Arch = Arch::PowerPc;

#[cfg(target_arch = "powerpc64")]
/// `target_arch` when building this crate: `powerpc64`
pub const TARGET_ARCH: Arch = Arch::PowerPc64;

#[cfg(target_arch = "riscv")]
/// `target_arch` when building this crate: `riscv`
pub const TARGET_ARCH: Arch = Arch::RiscV;

#[cfg(target_arch = "s390x")]
/// `target_arch` when building this crate: `s390x`
pub const TARGET_ARCH: Arch = Arch::S390X;

#[cfg(target_arch = "sparc")]
/// `target_arch` when building this crate: `sparc`
pub const TARGET_ARCH: Arch = Arch::Sparc;

#[cfg(target_arch = "sparc64")]
/// `target_arch` when building this crate: `sparc64`
pub const TARGET_ARCH: Arch = Arch::Sparc64;

#[cfg(target_arch = "wasm32")]
/// `target_arch` when building this crate: `wasm32`
pub const TARGET_ARCH: Arch = Arch::Wasm32;

#[cfg(target_arch = "x86")]
/// `target_arch` when building this crate: `x86`
pub const TARGET_ARCH: Arch = Arch::X86;

#[cfg(target_arch = "x86_64")]
/// `target_arch` when building this crate: `x86_64`
pub const TARGET_ARCH: Arch = Arch::X86_64;

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "asmjs",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "nvptx64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "riscv",
    target_arch = "s390x",
    target_arch = "sparc",
    target_arch = "sparc64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
/// `target_arch` when building this crate: unknown!
pub const TARGET_ARCH: Arch = Arch::Unknown;
