//! Package metadata

use crate::{error::Error, Map};
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Package metadata
pub type Metadata = Map<Key, Value>;

/// Keys for the `[metadata]` table
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Key(String);

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
