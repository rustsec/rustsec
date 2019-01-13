use core::fmt::{self, Display};

/// String to display in errors
const DISPLAY_STR: &str = "platforms::Error";

/// Error type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(DISPLAY_STR)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &'static str {
        DISPLAY_STR
    }
}
