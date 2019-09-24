//! Package metadata

use crate::{error::Error, Map};
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Package metadata
pub type Metadata = Map<Checksum, Hash>;

/// Package checksum info
// TODO(tarcieri): properly parse checksum strings
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Checksum(String);

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Checksum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(Checksum(s.to_owned()))
    }
}

impl<'de> Deserialize<'de> for Checksum {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        String::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}

impl Serialize for Checksum {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

/// Package hashes
// TODO(tarcieri): properly parse package hashes
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hash(String);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Hash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(Hash(s.to_owned()))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        String::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}

impl Serialize for Hash {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
