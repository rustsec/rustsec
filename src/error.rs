//! Error types used by this crate

use failure::{Backtrace, Context, Fail};
use reqwest;
use std::fmt::{self, Display};
use std::io;
use std::str::Utf8Error;
use toml;

/// Error type
#[derive(Debug)]
pub struct Error {
    /// Contextual information about the error
    inner: Context<ErrorKind>,

    /// Description of the error providing additional information
    description: String,
}

impl Error {
    /// Create a new error with the given description
    pub fn new<S: ToString>(kind: ErrorKind, description: &S) -> Self {
        Self {
            inner: Context::new(kind),
            description: description.to_string(),
        }
    }

    /// Obtain the inner `ErrorKind` for this error
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self.inner, &self.description)
    }
}

/// Custom error type for this library
#[derive(Copy, Clone, Debug, Eq, Fail, PartialEq)]
pub enum ErrorKind {
    /// An error occurred performing an I/O operation (e.g. network, file)
    #[fail(display = "I/O operation failed")]
    Io,

    /// Advisory database server responded with an error
    #[fail(display = "invalid response")]
    ServerResponse,

    /// Couldn't parse response data
    #[fail(display = "couldn't parse data")]
    Parse,

    /// Response data is missing an expected attribute
    #[fail(display = "expected attribute missing")]
    MissingAttribute,

    /// Response data contains an attributed which is the wrong type or otherwise invalid
    #[fail(display = "attribute is not the expected type/format")]
    InvalidAttribute,

    /// Version requirement is not properly formed
    #[fail(display = "malformatted version requirement")]
    MalformedVersion,
}

/// Create a new error (of a given enum variant) with a formatted message
macro_rules! err {
    ($kind:path, $msg:expr) => {
        ::error::Error::new(
            $kind,
            &$msg.to_string()
        )
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        err!($kind, &format!($fmt, $($arg)+))
    };
}

/// Create and return an error with a formatted message
macro_rules! fail {
    ($kind:path, $msg:expr) => {
        return Err(err!($kind, $msg).into());
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        fail!($kind, &format!($fmt, $($arg)+));
    };
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        err!(ErrorKind::Io, &other)
    }
}

impl From<reqwest::Error> for Error {
    fn from(other: reqwest::Error) -> Self {
        err!(ErrorKind::Io, &other)
    }
}

impl From<Utf8Error> for Error {
    fn from(other: Utf8Error) -> Self {
        err!(ErrorKind::Parse, &other)
    }
}

impl From<toml::de::Error> for Error {
    fn from(other: toml::de::Error) -> Self {
        err!(ErrorKind::Parse, &other)
    }
}
