use serde::{de::Error as DeError, Deserialize, Deserializer};

use error::Error;

/// Keywords on advisories, similar to Cargo keywords
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Keyword(String);

impl Keyword {
    /// Create a new keyword
    // TODO: validate keywords according to Cargo-like rules
    pub fn new<S: Into<String>>(keyword: S) -> Result<Self, Error> {
        Ok(Keyword(keyword.into()))
    }

    /// Borrow this keyword as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'de> Deserialize<'de> for Keyword {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{}", e)))
    }
}