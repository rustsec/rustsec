//! Error types

use std::fmt;

/// Create a new error (of a given enum variant) with a formatted message
macro_rules! format_err {
    ($kind:path, $msg:expr) => {
        crate::error::Error::new(
            $kind,
            Some($msg.to_string())
        )
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
       format_err!($kind, format!($fmt, $($arg)+))
    };
}

/// Create and return an error with a formatted message
macro_rules! fail {
    ($kind:path, $msg:expr) => {
        return Err(format_err!($kind, $msg))
    };
    ($kind:path, $fmt:expr, $($arg:tt)+) => {
        fail!($kind, format!($fmt, $($arg)+))
    };
}

/// Error type
#[derive(Clone, Debug)]
pub struct Error {
    /// Kinds of errors
    kind: ErrorKind,

    /// Message to associate with this error
    msg: Option<String>,
}

impl Error {
    /// Create a new error of this kind
    pub fn new(kind: ErrorKind, msg: Option<String>) -> Self {
        Error { kind, msg }
    }

    /// Get this error's kind
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref msg) = self.msg {
            write!(f, "{}: {}", self.kind, msg)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl std::error::Error for Error {}

/// Kinds of errors
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Parse errors
    Parse,

    /// Unsupported CVSS version
    Version,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Parse => write!(f, "parse error"),
            ErrorKind::Version => write!(f, "unsupported CVSS version"),
        }
    }
}
