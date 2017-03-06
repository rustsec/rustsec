//! Error types used by this crate

use std::{fmt, result};
use std::error::Error as StdError;

/// Custom error type for this library
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    /// An error occurred performing an I/O operation (e.g. network, file)
    IO,

    /// Advisory database server responded with an error
    ServerResponse,

    /// Couldn't parse response data
    Parse,

    /// Response data is missing an expected attribute
    MissingAttribute,

    /// Response data contains an attributed which is the wrong type or otherwise invalid
    InvalidAttribute,

    /// Version requirement is not properly formed
    MalformedVersion,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO => "I/O operation failed",
            Error::ServerResponse => "invalid response",
            Error::Parse => "couldn't parse data",
            Error::MissingAttribute => "expected attribute missing",
            Error::InvalidAttribute => "attribute is not the expected type/format",
            Error::MalformedVersion => "malformatted version requirement",
        }
    }
}

/// Custom result type for this library
pub type Result<T> = result::Result<T, Error>;
