//! Terminal status handling.
//!
//! Presently provides a Cargo-like visual style. Hopefully in future versions
//! this can be made configurable.
//!
//! # `status_ok!`: Successful status messages
//!
//! ```norun
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//! # }
//! ```
//!
//! # `status_err!`: Error messages
//!
//! ```norun
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print an error message
//! status_err!("something bad happened");
//! # }
//! ```
//!
//! # `status_attr_ok!`: Successful attributes
//!
//! ```norun
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//! # }
//! ```
//!
//! # `status_attr_error!`: Error attributes
//!
//! ```norun
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! # }
//! ```

use lazy_static::lazy_static;
use std::{
    io::{self, Write},
    sync::Mutex,
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

lazy_static! {
    /// Output stream
    pub static ref STREAM: Mutex<Option<StandardStream>> = Mutex::new(None);
}

/// Initialize the output terminal
pub(super) fn init(color_config: &str, use_stdout: bool) {
    let color_choice = match color_config {
        "always" => ColorChoice::Always,
        "never" => ColorChoice::Never,
        _ => ColorChoice::Auto,
    };

    let stream = if use_stdout {
        StandardStream::stdout(color_choice)
    } else {
        StandardStream::stderr(color_choice)
    };

    STREAM.lock().unwrap().replace(stream);
}

/// Print a success status message (in green if colors are enabled)
///
/// ```norun
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print a Cargo-like justified status to STDOUT
/// status_ok!("Loaded", "app loaded successfully");
/// # }
/// ```
#[macro_export]
macro_rules! status_ok {
    ($status:expr, $msg:expr) => {
        $crate::terminal::Status::new()
            .justified()
            .bold()
            .color(termcolor::Color::Green)
            .status($status)
            .print($msg)
            .unwrap();
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_ok!($status, format!($fmt, $($arg)+));
    };
}

/// Print an informational status message (in cyan if colors are enabled)
///
/// ```norun
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print a Cargo-like justified status to STDOUT
/// status_info!("Info", "you may care to know about");
/// # }
/// ```
#[macro_export]
macro_rules! status_info {
    ($status:expr, $msg:expr) => {
        $crate::terminal::Status::new()
            .justified()
            .bold()
            .color(termcolor::Color::Cyan)
            .status($status)
            .print($msg)
            .unwrap();
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_info!($status, format!($fmt, $($arg)+));
    };
}

/// Print a warning status message (in yellow if colors are enabled)
///
/// ```norun
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print a Cargo-like justified status to STDOUT
/// status_warn!("heads up, there's something you should know");
/// # }
/// ```
#[macro_export]
macro_rules! status_warn {
    ($msg:expr) => {
        $crate::terminal::Status::new()
            .bold()
            .color(termcolor::Color::Yellow)
            .status("warning:")
            .print($msg)
            .unwrap();
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status_warn!(format!($fmt, $($arg)+));
    };
}

/// Print an error message (in red if colors are enabled)
///
/// ```norun
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print an error message
/// status_err!("something bad happened");
/// # }
/// ```
#[macro_export]
macro_rules! status_err {
    ($msg:expr) => {
        $crate::terminal::Status::new()
            .bold()
            .color(termcolor::Color::Red)
            .status("error:")
            .print($msg)
            .unwrap();
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status_err!(format!($fmt, $($arg)+));
    };
}

/// Print a tab-delimited status attribute (in green if colors are enabled)
///
/// ```norun
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print an indented attribute to STDOUT
/// status_attr_ok!("good", "yep");
/// # }
/// ```
#[macro_export]
macro_rules! status_attr_ok {
    ($attr:expr, $msg:expr) => {
        // TODO(tarcieri): hax... use a better format string?
        let attr_delimited = if $attr.len() >= 7 {
            format!("{}:", $attr)
        } else {
            format!("{}:\t", $attr)
        };


        $crate::terminal::Status::new()
            .bold()
            .color(termcolor::Color::Green)
            .status(attr_delimited)
            .print($msg)
            .unwrap();
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_attr_ok!($attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in red if colors are enabled)
///
/// ```norun
/// # #[macro_use] extern crate abscissa;
/// # fn main() {
/// // Print an error attribute to STDERR
/// status_attr_err!("error", "yep");
/// # }
/// ```
#[macro_export]
macro_rules! status_attr_err {
    ($attr:expr, $msg:expr) => {
        // TODO(tarcieri): hax... use a better format string?
        let attr_delimited = if $attr.len() >= 7 {
            format!("{}:", $attr)
        } else {
            format!("{}:\t", $attr)
        };


        $crate::terminal::Status::new()
            .bold()
            .color(termcolor::Color::Red)
            .status(attr_delimited)
            .print($msg)
            .unwrap();
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_attr_err!($attr, format!($fmt, $($arg)+));
    }
}

/// Status message builder
#[derive(Clone, Debug, Default)]
pub struct Status {
    /// Should the status be justified?
    justified: bool,

    /// Should colors be bold?
    bold: bool,

    /// Color in which status should be displayed
    color: Option<Color>,

    /// Prefix of the status message (e.g. `Success`)
    status: Option<String>,
}

impl Status {
    /// Create a new status message with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Justify status on display
    pub fn justified(mut self) -> Self {
        self.justified = true;
        self
    }

    /// Make colors bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set the colors used to display this message
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }

    /// Set a status message to display
    pub fn status<S>(mut self, msg: S) -> Self
    where
        S: ToString,
    {
        self.status = Some(msg.to_string());
        self
    }

    /// Print the given message
    pub fn print<S>(self, msg: S) -> Result<(), io::Error>
    where
        S: AsRef<str>,
    {
        let stream = STREAM.lock().unwrap();
        let mut s = stream.as_ref().unwrap().lock();

        s.reset()?;
        s.set_color(ColorSpec::new().set_fg(self.color).set_bold(self.bold))?;

        if let Some(status) = self.status {
            if self.justified {
                write!(s, "{:>12}", status)?;
            } else {
                write!(s, "{}", status)?;
            }
        }

        s.reset()?;
        writeln!(s, " {}", msg.as_ref())?;
        s.flush()?;

        Ok(())
    }
}
