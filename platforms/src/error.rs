//! Error type

use core::fmt::{self, Display};

#[cfg(feature = "std")]
use std::error::Error as StdError;

/// Error type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("platforms::Error")
    }
}

#[cfg(feature = "std")]
impl StdError for Error {}
