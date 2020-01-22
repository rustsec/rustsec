//! Package metadata

use crate::{
    error::{Error, ErrorKind},
    lockfile::encoding::EncodableDependency,
    Checksum, Dependency, Map,
};
use serde::{de, ser, Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

/// Prefix of metadata keys for checksum entries
const CHECKSUM_PREFIX: &str = "checksum ";

/// Package metadata
pub type Metadata = Map<Key, Value>;

/// Keys for the `[metadata]` table
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Key(String);

impl Key {
    /// Create a metadata key for a checksum for the given dependency
    pub fn for_checksum(dep: &Dependency) -> Self {
        Key(format!("{}{}", CHECKSUM_PREFIX, dep))
    }

    /// Is this metadata key a checksum entry?
    pub fn is_checksum(&self) -> bool {
        self.0.starts_with(CHECKSUM_PREFIX)
    }

    /// Get the dependency for a particular checksum value (if applicable)
    pub fn checksum_dependency(&self) -> Result<Dependency, Error> {
        self.try_into()
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(Key(s.to_owned()))
    }
}

impl TryFrom<&Key> for Dependency {
    type Error = Error;

    fn try_from(key: &Key) -> Result<Dependency, Error> {
        if !key.is_checksum() {
            fail!(
                ErrorKind::Parse,
                "can only parse dependencies from `checksum` metadata"
            );
        }

        let dep = EncodableDependency::from_str(&key.as_ref()[CHECKSUM_PREFIX.len()..])?;
        (&dep).try_into()
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Serialize for Key {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

/// Values in the `[metadata]` table
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value(String);

impl Value {
    /// Get the associated checksum for this value (if applicable)
    pub fn checksum(&self) -> Result<Checksum, Error> {
        self.try_into()
    }
}

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(Value(s.to_owned()))
    }
}

impl TryFrom<&Value> for Checksum {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Checksum, Error> {
        value.as_ref().parse()
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Serialize for Value {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
