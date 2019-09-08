//! Error types

use abscissa_core::err;
use failure::Fail;
use std::{fmt, io};

/// Error type
#[derive(Debug)]
pub struct Error(abscissa_core::Error<ErrorKind>);

impl Error {
    /// Get the kind of error that occurred
    pub fn kind(&self) -> ErrorKind {
        *self.0.kind()
    }
}

/// Kinds of errors
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    /// Error in configuration file
    #[fail(display = "config error")]
    Config,

    /// Input/output error
    #[fail(display = "I/O error")]
    Io,

    /// Parse errors
    #[fail(display = "parse error")]
    Parse,

    /// Repository errors
    #[fail(display = "git repo error")]
    Repo,

    /// Version errors
    #[fail(display = "version error")]
    Version,

    /// Other kinds of errors
    #[fail(display = "other error")]
    Other,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<abscissa_core::Error<ErrorKind>> for Error {
    fn from(other: abscissa_core::Error<ErrorKind>) -> Self {
        Error(other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        err!(ErrorKind::Io, other).into()
    }
}

impl From<rustsec::Error> for Error {
    fn from(other: rustsec::Error) -> Self {
        let kind = match other.kind() {
            rustsec::ErrorKind::Io => ErrorKind::Io,
            rustsec::ErrorKind::Parse => ErrorKind::Parse,
            rustsec::ErrorKind::Repo => ErrorKind::Repo,
            rustsec::ErrorKind::Version => ErrorKind::Version,
            _ => ErrorKind::Other,
        };

        Error(err!(kind, "{}", other))
    }
}
