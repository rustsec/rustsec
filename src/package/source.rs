//! Rust package sources

use crate::Error;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Source for a Rust `[[package]]`
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Source(String);

impl Source {
    /// Get source as an `&str`
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Source {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        // TODO(tarcieri): ensure source is valid
        Ok(Source(s.into()))
    }
}
