/// `target_env`: Target enviroment that disambiguates the target platform by ABI / libc.
/// This value is closely related to the fourth element of the platform target triple,
/// though it is not identical. For example, embedded ABIs such as `gnueabihf` will simply
/// define `target_env` as `"gnu"` (i.e. `target::Env::GNU`)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Env {
    /// `gnu`: The GNU C Library (glibc)
    GNU,

    /// `msvc`: Microsoft Visual C(++)
    MSVC,

    /// `musl`: Clean, efficient, standards-conformant libc implementation.
    Musl,

    /// `uclibc`: C library for developing embedded Linux systems
    #[allow(non_camel_case_types)]
    uClibc,
}

impl Env {
    /// String representing this environment which matches `#[cfg(target_env)]`
    pub fn as_str(self) -> &'static str {
        match self {
            Env::GNU => "gnu",
            Env::MSVC => "msvc",
            Env::Musl => "musl",
            Env::uClibc => "uclibc",
        }
    }
}

// Detect and expose `target_env` as a constant
// Whether this is a good idea is somewhat debatable

#[cfg(target_env = "gnu")]
/// `target_env` when building this crate: `gnu`
pub const TARGET_ENV: Option<Env> = Some(Env::GNU);

#[cfg(target_env = "msvc")]
/// `target_env` when building this crate: `msvc`
pub const TARGET_ENV: Option<Env> = Some(Env::MSVC);

#[cfg(target_env = "musl")]
/// `target_env` when building this crate: `musl`
pub const TARGET_ENV: Option<Env> = Some(Env::MUSL);

#[cfg(target_env = "uclibc")]
/// `target_env` when building this crate: `uclibc`
pub const TARGET_ENV: Option<Env> = Some(Env::MUSL);

#[cfg(
    not(
        any(
            target_env = "gnu",
            target_env = "msvc",
            target_env = "musl",
            target_env = "uclibc",
        )
    )
)]
/// `target_env` when building this crate: none
pub const TARGET_ENV: Option<Env> = None;
