//! Rust target environments

use crate::error::Error;
use core::{fmt, str::FromStr};
use std::string::String;

#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};

/// `target_env`: target enviroment that disambiguates the target platform by ABI / libc.
/// This value is closely related to the fourth element of the platform target triple,
/// though it is not identical. For example, embedded ABIs such as `gnueabihf` will simply
/// define `target_env` as `"gnu"` (i.e. `target::Env::GNU`)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Env {
    /// `gnu`: The GNU C Library (glibc)
    Gnu,

    /// `msvc`: Microsoft Visual C(++)
    Msvc,

    /// `musl`: Clean, efficient, standards-conformant libc implementation.
    Musl,

    /// `sgx`: Intel Software Guard Extensions (SGX) Enclave
    Sgx,

    /// `uclibc`: C library for developing embedded Linux systems
    UClibc,

    /// Unknown target environment
    Unknown,
}

impl Env {
    /// String representing this environment which matches `#[cfg(target_env)]`
    pub fn as_str(self) -> &'static str {
        match self {
            Env::Gnu => "gnu",
            Env::Msvc => "msvc",
            Env::Musl => "musl",
            Env::Sgx => "sgx",
            Env::UClibc => "uclibc",
            Env::Unknown => "unknown",
        }
    }
}

impl FromStr for Env {
    type Err = Error;

    /// Create a new `Env` from the given string
    fn from_str(env_name: &str) -> Result<Self, Self::Err> {
        let env = match env_name {
            "gnu" => Env::Gnu,
            "msvc" => Env::Msvc,
            "musl" => Env::Musl,
            "sgx" => Env::Sgx,
            "uclibc" => Env::UClibc,
            _ => return Err(Error),
        };

        Ok(env)
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Env {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Env {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(String::deserialize(deserializer)?
            .parse()
            .unwrap_or(Env::Unknown))
    }
}

// Detect and expose `target_env` as a constant
// Whether this is a good idea is somewhat debatable

#[cfg(target_env = "gnu")]
/// `target_env` when building this crate: `gnu`
pub const TARGET_ENV: Option<Env> = Some(Env::Gnu);

#[cfg(target_env = "msvc")]
/// `target_env` when building this crate: `msvc`
pub const TARGET_ENV: Option<Env> = Some(Env::Msvc);

#[cfg(target_env = "musl")]
/// `target_env` when building this crate: `musl`
pub const TARGET_ENV: Option<Env> = Some(Env::Musl);

#[cfg(target_env = "sgx")]
/// `target_env` when building this crate: `sgx`
pub const TARGET_ENV: Option<Env> = Some(Env::Sgx);

#[cfg(target_env = "uclibc")]
/// `target_env` when building this crate: `uclibc`
pub const TARGET_ENV: Option<Env> = Some(Env::UClibc);

#[cfg(not(any(
    target_env = "gnu",
    target_env = "msvc",
    target_env = "musl",
    target_env = "sgx",
    target_env = "uclibc",
)))]
/// `target_env` when building this crate: none
pub const TARGET_ENV: Option<Env> = None;


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::target::Env;

    #[test]
    fn valid_os_tests() {
        assert!(Env::from_str("gnu").is_ok());
        assert!(Env::from_str("uclibc").is_ok());
    }

    #[test]
    fn invalid_os_tests() {
        assert!(Env::from_str("").is_err());
        assert!(Env::from_str(" ").is_err());
        assert!(Env::from_str("derp").is_err());
        assert!(Env::from_str("***").is_err());
        assert!(Env::from_str("unknown").is_err());
    }
}
