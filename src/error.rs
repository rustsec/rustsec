//! Error types used by this crate

use std::{
    fmt::{self, Display},
    io,
    str::Utf8Error,
};
use thiserror::Error;

/// Create a new error (of a given enum variant) with a formatted message
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

/// Error type
#[derive(Debug)]
pub struct Error {
    /// Kind of error
    kind: ErrorKind,

    /// Message providing additional information
    msg: String,
}

impl Error {
    /// Create a new error with the given description
    pub fn new<S: ToString>(kind: ErrorKind, description: &S) -> Self {
        Self {
            kind,
            msg: description.to_string(),
        }
    }

    /// Obtain the inner `ErrorKind` for this error
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", &self.kind, &self.msg)
    }
}

impl std::error::Error for Error {}

/// Custom error type for this library
#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum ErrorKind {
    /// Invalid argument or parameter
    #[error("bad parameter")]
    BadParam,

    /// Error performing an automatic fix
    #[cfg(feature = "fix")]
    #[error("fix failed")]
    Fix,

    /// An error occurred performing an I/O operation (e.g. network, file)
    #[error("I/O operation failed")]
    Io,

    /// Not found
    #[error("not found")]
    NotFound,

    /// Couldn't parse response data
    #[error("parse error")]
    Parse,

    /// Registry-related error
    #[error("registry")]
    Registry,

    /// Git operation failed
    #[error("git operation failed")]
    Repo,

    /// Errors related to versions
    #[error("bad version")]
    Version,
}

impl From<Utf8Error> for Error {
    fn from(other: Utf8Error) -> Self {
        format_err!(ErrorKind::Parse, &other)
    }
}

#[cfg(feature = "fix")]
impl From<cargo_edit::Error> for Error {
    fn from(other: cargo_edit::Error) -> Self {
        format_err!(ErrorKind::Fix, &other)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(other: chrono::ParseError) -> Self {
        format_err!(ErrorKind::Parse, &other)
    }
}

impl From<fmt::Error> for Error {
    fn from(other: fmt::Error) -> Self {
        format_err!(ErrorKind::Io, &other)
    }
}

#[cfg(feature = "fetch")]
impl From<git2::Error> for Error {
    fn from(other: git2::Error) -> Self {
        format_err!(ErrorKind::Repo, &other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        format_err!(ErrorKind::Io, &other)
    }
}

#[cfg(feature = "fetch")]
impl From<crates_index::Error> for Error {
    fn from(other: crates_index::Error) -> Self {
        format_err!(ErrorKind::Registry, "{}", other)
    }
}

impl From<semver::SemVerError> for Error {
    fn from(other: semver::SemVerError) -> Self {
        format_err!(ErrorKind::Version, &other)
    }
}

impl From<semver::ReqParseError> for Error {
    fn from(other: semver::ReqParseError) -> Self {
        format_err!(ErrorKind::Version, &other)
    }
}

impl From<toml::de::Error> for Error {
    fn from(other: toml::de::Error) -> Self {
        format_err!(ErrorKind::Parse, &other)
    }
}
