/// `target_arch`: Target CPU architecture
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Arch {
    /// `aarch64`: ARMv8 64-bit architecture
    AARCH64,

    /// `arm`: 32-bit ARM architecture
    ARM,

    /// `asm`: asm.js output
    ASMJS,

    /// `mips`: 32-bit MIPS CPU architecture
    MIPS,

    /// `mips64`: 32-bit MIPS CPU architecture
    MIPS64,

    /// `msp430`: 16-bit MSP430 microcontrollers
    MSP430,

    /// `powerpc`: 32-bit POWERPC platform
    POWERPC,

    /// `powerpc64`: 64-bit POWERPC platform
    POWERPC64,

    /// `riscv`: RISC-V CPU architecture
    RISCV,

    /// `s390x`: 64-bit IBM z/Architecture
    S390X,

    /// `sparc64`: 64-bit SPARC CPU architecture
    SPARC64,

    /// `wasm32`: Web Assembly (32-bit)
    WASM32,

    /// `x86`: Generic x86 architecture
    X86,

    /// `x86_64`: "AMD64" architecture
    X86_64,

    /// Unknown CPU architecture
    Unknown,
}

impl Arch {
    /// String representing this target architecture which matches `cfg(target_arch)`
    pub fn as_str(self) -> &'static str {
        match self {
            Arch::AARCH64 => "aarch64",
            Arch::ARM => "arm",
            Arch::ASMJS => "asmjs",
            Arch::MIPS => "mips",
            Arch::MIPS64 => "mips64",
            Arch::MSP430 => "msp430",
            Arch::POWERPC => "powerpc",
            Arch::POWERPC64 => "powerpc64",
            Arch::RISCV => "riscv",
            Arch::S390X => "s390x",
            Arch::SPARC64 => "sparc64",
            Arch::WASM32 => "wasm32",
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::Unknown => "unknown",
        }
    }
}

// Detect and expose `target_arch` as a constant
// Whether this is a good idea is somewhat debatable

#[cfg(target_arch = "aarch64")]
/// `target_arch` when building this crate: `x86_64`
pub const TARGET_ARCH: Arch = Arch::AARCH64;

#[cfg(target_arch = "arm")]
/// `target_arch` when building this crate: `arm`
pub const TARGET_ARCH: Arch = Arch::ARM;

#[cfg(target_arch = "asmjs")]
/// `target_arch` when building this crate: `asmjs`
pub const TARGET_ARCH: Arch = Arch::ASMJS;

#[cfg(target_arch = "mips")]
/// `target_arch` when building this crate: `mips`
pub const TARGET_ARCH: Arch = Arch::MIPS;

#[cfg(target_arch = "mips64")]
/// `target_arch` when building this crate: `mips64`
pub const TARGET_ARCH: Arch = Arch::MIPS64;

#[cfg(target_arch = "msp430")]
/// `target_arch` when building this crate: `msp430`
pub const TARGET_ARCH: Arch = Arch::MSP430;

#[cfg(target_arch = "powerpc")]
/// `target_arch` when building this crate: `powerpc`
pub const TARGET_ARCH: Arch = Arch::POWERPC;

#[cfg(target_arch = "powerpc64")]
/// `target_arch` when building this crate: `powerpc64`
pub const TARGET_ARCH: Arch = Arch::POWERPC64;

#[cfg(target_arch = "riscv")]
/// `target_arch` when building this crate: `riscv`
pub const TARGET_ARCH: Arch = Arch::RISCV;

#[cfg(target_arch = "s390x")]
/// `target_arch` when building this crate: `s390x`
pub const TARGET_ARCH: Arch = Arch::S390X;

#[cfg(target_arch = "sparc64")]
/// `target_arch` when building this crate: `sparc64`
pub const TARGET_ARCH: Arch = Arch::SPARC64;

#[cfg(target_arch = "wasm32")]
/// `target_arch` when building this crate: `wasm32`
pub const TARGET_ARCH: Arch = Arch::WASM32;

#[cfg(target_arch = "x86")]
/// `target_arch` when building this crate: `x86`
pub const TARGET_ARCH: Arch = Arch::X86;

#[cfg(target_arch = "x86_64")]
/// `target_arch` when building this crate: `x86_64`
pub const TARGET_ARCH: Arch = Arch::X86_64;

#[cfg(
    not(
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "asmjs",
            target_arch = "mips",
            target_arch = "mips64",
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "riscv",
            target_arch = "s390x",
            target_arch = "sparc64",
            target_arch = "wasm32",
            target_arch = "x86",
            target_arch = "x86_64"
        )
    )
)]
/// `target_arch` when building this crate: unknown!
pub const TARGET_ARCH: Arch = Arch::Unknown;
