//! Error types used by this crate

use std::{
    fmt::{self, Display},
    io,
    str::Utf8Error,
};
use thiserror::Error;

#[cfg(feature = "git")]
use tame_index::external::gix;

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
        return Err(format_err!($kind, $msg).into())
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        fail!($kind, &format!($fmt, $($arg)+))
    };
}

/// Result alias with the `rustsec` crate's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

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
#[non_exhaustive]
pub enum ErrorKind {
    /// Invalid argument or parameter
    #[error("bad parameter")]
    BadParam,

    /// Error performing an automatic fix
    #[cfg(feature = "fix")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fix")))]
    #[error("fix failed")]
    Fix,

    /// An error occurred performing an I/O operation (e.g. network, file)
    #[error("I/O operation failed")]
    Io,

    /// Not found
    #[error("not found")]
    NotFound,

    /// Unable to acquire filesystem lock
    #[error("unable to acquire filesystem lock")]
    LockTimeout,

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
#[cfg_attr(docsrs, doc(cfg(feature = "fix")))]
impl From<cargo_edit::Error> for Error {
    fn from(other: cargo_edit::Error) -> Self {
        format_err!(ErrorKind::Fix, &other)
    }
}

impl From<cargo_lock::Error> for Error {
    fn from(other: cargo_lock::Error) -> Self {
        format_err!(ErrorKind::Io, &other)
    }
}

impl From<fmt::Error> for Error {
    fn from(other: fmt::Error) -> Self {
        format_err!(ErrorKind::Io, &other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        format_err!(ErrorKind::Io, &other)
    }
}

impl Error {
    /// Converts from [`tame_index::Error`] to our `Error`.
    ///
    /// This is a separate function instead of a `From` impl
    /// because a trait impl would leak into the public API,
    /// and we need to keep it private because `tame_index` semver
    /// will be bumped frequently and we don't want to bump `rustsec` semver
    /// every time it changes.
    #[cfg(feature = "git")]
    pub(crate) fn from_tame(err: tame_index::Error) -> Self {
        // Separate lock timeouts into their own LockTimeout variant.
        match err {
            tame_index::Error::Lock(lock_err) => format_err!(ErrorKind::LockTimeout, "{}", lock_err),
            other => format_err!(ErrorKind::Registry, "{}", other),
        }
    }

    /// Converts from [`gix::lock::acquire::Error`] to our `Error`.
    ///
    /// This is a separate function instead of a `From` impl
    /// because a trait impl would leak into the public API,
    /// and we need to keep it private because `gix` semver
    /// will be bumped frequently and we don't want to bump `rustsec` semver
    /// every time it changes.
    #[cfg(feature = "git")]
    pub(crate) fn from_gix_lock(other: gix::lock::acquire::Error) -> Self {
        match other {
            gix::lock::acquire::Error::Io(e) => {
                format_err!(ErrorKind::Repo, "failed to aquire directory lock: {}", e)
            }
            gix::lock::acquire::Error::PermanentlyLocked {
                resource_path,
                mode: _,
                attempts,
            } => format_err!(
                ErrorKind::LockTimeout,
                "directory \"{:?}\" still locked after {} attempts",
                resource_path,
                attempts,
            ),
        }
    }

    /// Converts from [`toml::de::Error`] to our `Error`.
    ///
    /// This is used so rarely that there is no need to `impl From`,
    /// and this way we can avoid leaking it into the public API.
    pub(crate) fn from_toml(other: toml::de::Error) -> Self {
        format_err!(crate::ErrorKind::Parse, &other)
    }
}
