//! Error types

use std::{
    fmt::{self, Display},
    io,
};

/// Create error with a formatted message
macro_rules! format_err {
    ($kind:path, $msg:expr) => {
        crate::error::Error::new(
            $kind,
            &$msg.to_string()
        )
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        format_err!($kind, &format!($fmt, $($arg)+))
    };
}

/// Create and return an error with a formatted message
macro_rules! fail {
    ($kind:path, $msg:expr) => {
        return Err(format_err!($kind, $msg).into());
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        fail!($kind, &format!($fmt, $($arg)+));
    };
}

/// Custom error type for this library
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// An error occurred performing an I/O operation (e.g. network, file)
    Io,

    /// Couldn't parse response data
    Parse,

    /// Errors related to versions
    Version,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ErrorKind::Io => "I/O operation failed",
            ErrorKind::Parse => "parse error",
            ErrorKind::Version => "bad version",
        };

        write!(f, "{}", msg)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        format_err!(ErrorKind::Io, &other)
    }
}

impl From<semver::Error> for Error {
    fn from(other: semver::Error) -> Self {
        format_err!(ErrorKind::Version, &other)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(other: std::num::ParseIntError) -> Self {
        format_err!(ErrorKind::Parse, &other)
    }
}

impl From<toml::de::Error> for Error {
    fn from(other: toml::de::Error) -> Self {
        format_err!(ErrorKind::Parse, &other)
    }
}

/// Error type
#[derive(Debug)]
pub struct Error {
    /// Kind of error
    kind: ErrorKind,

    /// Message providing additional information
    msg: String,
}

impl Error {
    /// Create a new error with the given message
    pub fn new<S: ToString>(kind: ErrorKind, msg: &S) -> Self {
        Self {
            kind,
            msg: msg.to_string(),
        }
    }

    /// Obtain the inner `ErrorKind` for this error
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Obtain the associated error message
    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", &self.kind, &self.msg)
    }
}

impl std::error::Error for Error {}
