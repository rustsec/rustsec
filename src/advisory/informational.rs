//! Informational advisories: ones which don't represent an immediate security
//! threat, but something users of a crate should be warned of/aware of

use crate::error::Error;
use serde::{de, ser, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Categories of informational vulnerabilities
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Informational {
    /// Security notices for a crate which are published on <https://rustsec.org>
    /// but don't represent a vulnerability in a crate itself.
    Notice,

    /// Crate is unmaintained / abandoned
    Unmaintained,

    /// Other types of informational advisories: left open-ended to add
    /// more of them in the future.
    Other(String),
}

impl Informational {
    /// Get a `str` representing an innformationnal category
    pub fn as_str(&self) -> &str {
        match self {
            Informational::Notice => "notice",
            Informational::Unmaintained => "unmaintained",
            Informational::Other(other) => other,
        }
    }
}

impl fmt::Display for Informational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Informational {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "notice" => Informational::Notice,
            "unmaintained" => Informational::Unmaintained,
            other => Informational::Other(other.to_owned()),
        })
    }
}

impl<'de> Deserialize<'de> for Informational {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}

impl Serialize for Informational {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::Informational;

    #[test]
    fn parse_notice() {
        let notice = "notice".parse::<Informational>().unwrap();
        assert_eq!(Informational::Notice, notice);
        assert_eq!("notice", notice.as_str());
    }

    #[test]
    fn parse_unmaintainend() {
        let unmaintained = "unmaintained".parse::<Informational>().unwrap();
        assert_eq!(Informational::Unmaintained, unmaintained);
        assert_eq!("unmaintained", unmaintained.as_str());
    }

    #[test]
    fn parse_other() {
        let other = "foobar".parse::<Informational>().unwrap();
        assert_eq!(Informational::Other("foobar".to_owned()), other);
        assert_eq!("foobar", other.as_str());
    }
}
