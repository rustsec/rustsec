//! RustSec Vulnerability Categories

use crate::error::{Error, ErrorKind};
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// RustSec Vulnerability Categories
///
/// The RustSec project maintains its own categorization system for
/// vulnerabilities according to our [criteria for acceptable advisories][1].
///
/// This type represents the present list of allowable vulnerability types for
/// which we allow advisories to be filed.
///
/// [1]: https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md#criteria
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Category {
    /// Cryptography Failure (e.g. confidentiality breakage, integrity
    /// breakage, key leakage)
    CryptographicFailure,

    /// Attacks that cause crashes or excess resource consumption such that
    /// software ceases to function normally, notably panics in code that is
    /// advertised as "panic-free" (particularly in format parsers for
    /// untrusted data)
    DenialOfService,

    /// Disclosure of local files (a.k.a. "directory traversal")
    FileDisclosure,

    /// Mishandled escaping allowing an attacker to execute code or perform
    /// otherwise unexpected operations, e.g. shell escaping, SQL injection, XSS.
    FormatInjection,

    /// Memory unsafety vulnerabilities allowing an attacker to write to
    /// unintended locations in memory.
    MemoryCorruption,

    /// Read-only memory safety vulnerabilities which unintentionally expose data.
    MemoryExposure,

    /// Attacks which bypass authentication and/or authorization systems,
    /// allowing the attacker to obtain unintended privileges.
    PrivilegeEscalation,

    /// Execution of arbitrary code allowing an attacker to gain partial or
    /// total control of an impacted computer system.
    RemoteCodeExecution,
}

impl Category {
    /// Get the short "kebab case" identifier for a category
    pub fn name(self) -> &'static str {
        match self {
            Category::CryptographicFailure => "crypto",
            Category::DenialOfService => "dos",
            Category::FileDisclosure => "lfd",
            Category::FormatInjection => "format-injection",
            Category::MemoryCorruption => "memory-corruption",
            Category::MemoryExposure => "memory-exposure",
            Category::PrivilegeEscalation => "privilege-escalation",
            Category::RemoteCodeExecution => "rce",
        }
    }

    /// Get a brief text description of the category
    pub fn description(self) -> &'static str {
        match self {
            Category::CryptographicFailure => "cryptographic failure",
            Category::DenialOfService => "denial-of-service (DoS)",
            Category::FileDisclosure => "file disclosure (LFD)",
            Category::FormatInjection => "format injection",
            Category::MemoryCorruption => "memory corruption",
            Category::MemoryExposure => "memory exposure",
            Category::PrivilegeEscalation => "privilege escalation",
            Category::RemoteCodeExecution => "remote code execution (RCE)",
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "crypto" => Category::CryptographicFailure,
            "dos" => Category::DenialOfService,
            "lfd" => Category::FileDisclosure,
            "format-injection" => Category::FormatInjection,
            "memory-corruption" => Category::MemoryCorruption,
            "memory-exposure" => Category::MemoryExposure,
            "privilege-escalation" => Category::PrivilegeEscalation,
            "rce" => Category::RemoteCodeExecution,
            other => fail!(ErrorKind::Parse, "invalid category: {}", other),
        })
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}

impl Serialize for Category {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
