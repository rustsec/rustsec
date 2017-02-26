use std::{fmt, result};
use std::error::Error as StdError;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    Request,
    Response,
    Parse,
    MissingAttribute,
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
            Error::Request => "network request failed",
            Error::Response => "invalid response",
            Error::Parse => "couldn't parse data",
            Error::MissingAttribute => "expected attribute missing",
            Error::MalformedVersion => "malformatted version requirement",
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
