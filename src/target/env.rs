use core::str::FromStr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;

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

    /// `sgx`: Intel Software Guard Extensions (SGX) Enclave
    SGX,

    /// `uclibc`: C library for developing embedded Linux systems
    #[allow(non_camel_case_types)]
    uClibc,

    /// Unknown target environment
    Unknown,
}

impl Env {
    /// String representing this environment which matches `#[cfg(target_env)]`
    pub fn as_str(self) -> &'static str {
        match self {
            Env::GNU => "gnu",
            Env::MSVC => "msvc",
            Env::Musl => "musl",
            Env::SGX => "sgx",
            Env::uClibc => "uclibc",
            Env::Unknown => "unknown",
        }
    }
}

impl FromStr for Env {
    type Err = Error;

    /// Create a new `Env` from the given string
    fn from_str(env_name: &str) -> Result<Self, Self::Err> {
        let env = match env_name {
            "gnu" => Env::GNU,
            "msvc" => Env::MSVC,
            "musl" => Env::Musl,
            "sgx" => Env::SGX,
            "uclibc" => Env::uClibc,
            _ => return Err(Error),
        };

        Ok(env)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Env {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Env {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_str(<&str>::deserialize(deserializer)?).unwrap_or(Env::Unknown))
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
pub const TARGET_ENV: Option<Env> = Some(Env::Musl);

#[cfg(target_env = "sgx")]
/// `target_env` when building this crate: `sgx`
pub const TARGET_ENV: Option<Env> = Some(Env::SGX);

#[cfg(target_env = "uclibc")]
/// `target_env` when building this crate: `uclibc`
pub const TARGET_ENV: Option<Env> = Some(Env::uClibc);

#[cfg(not(any(
    target_env = "gnu",
    target_env = "msvc",
    target_env = "musl",
    target_env = "sgx",
    target_env = "uclibc",
)))]
/// `target_env` when building this crate: none
pub const TARGET_ENV: Option<Env> = None;
