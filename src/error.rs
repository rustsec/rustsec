//! Error types

use abscissa_core::err;
use failure::Fail;
use std::{fmt, io};

/// Error type
#[derive(Debug)]
pub struct Error(abscissa_core::Error<ErrorKind>);

/// Kinds of errors
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    /// Error in configuration file
    #[fail(display = "config error")]
    Config,

    /// Input/output error
    #[fail(display = "I/O error")]
    Io,
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
