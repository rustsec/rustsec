//! Terminal handling code
//!
//! Some portions borrowed from the Cargo project: https://github.com/rust-lang/cargo
//! These portions are redistributed under the same license as Cargo (shared by cargo-audit itself)

use atty::{self, Stream};
use std::{
    cell::RefCell,
    fmt,
    io::{self, prelude::*},
    sync::Mutex,
};
use term::{
    self,
    color::{Color, BLACK},
    Attr, Terminal as TermTerminal, TerminfoTerminal,
};

lazy_static! {
    static ref SHELL: Mutex<RefCell<Option<Shell>>> = Mutex::new(RefCell::new(None));
}

/// Initialize the shell
pub fn init(color_config: &str, use_stdout: bool) {
    let config = ShellConfig {
        color_config: match color_config {
            "always" => ColorConfig::Always,
            "never" => ColorConfig::Never,
            _ => ColorConfig::Auto,
        },
        tty: atty::is(Stream::Stdout),
    };

    let shell = Shell::new(
        || {
            if use_stdout {
                Box::new(io::stdout())
            } else {
                Box::new(io::stderr())
            }
        },
        config,
    );
    SHELL.lock().unwrap().replace(Some(shell));
}

/// Say a status message with the given color
pub fn status<T, U>(color: Color, status: T, message: U, justified: bool)
where
    T: fmt::Display,
    U: fmt::Display,
{
    let shell = SHELL.lock().unwrap();
    shell
        .borrow_mut()
        .as_mut()
        .expect("shell not configured!")
        .status(color, status, message, justified)
        .unwrap();
}

/// Print an attribute of an advisory
macro_rules! attribute {
    ($attr:expr, $msg:expr) => {
        ::shell::status(
            ::term::color::RED,
            if $attr.len() >= 7 {
                format!("{}:", $attr)
            } else {
                format!("{}:\t", $attr)
            },
            $msg,
            false,
        );
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        attribute!($attr, format!($fmt, $($arg)+));
    }
}

/// Print a success status message (in green if colors are enabled)
macro_rules! status_ok {
    ($status:expr, $msg:expr) => {
        ::shell::status(::term::color::GREEN, $status, $msg, true);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        status_ok!($status, format!($fmt, $($arg)+));
    };
}

/// Print an error message (in red if colors are enabled)
macro_rules! status_error {
    ($msg:expr) => {
        ::shell::status(::term::color::RED, "error:", $msg, false);
    };
    ($fmt:expr, $($arg:tt)+) => {
        status_error!(format!($fmt, $($arg)+));
    };
}

/// Color configuration
#[derive(Clone, Copy, PartialEq)]
enum ColorConfig {
    /// Pick colors automatically based on whether we're using a TTY
    Auto,

    /// Always use colors
    Always,

    /// Never use colors
    Never,
}

impl fmt::Display for ColorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorConfig::Auto => "auto",
            ColorConfig::Always => "always",
            ColorConfig::Never => "never",
        }.fmt(f)
    }
}

/// Shell configuration options
#[derive(Clone, Copy)]
struct ShellConfig {
    /// Color configuration
    pub color_config: ColorConfig,

    /// Are we using a TTY?
    pub tty: bool,
}
enum Terminal {
    NoColor(Box<Write + Send>),
    Colored(Box<term::Terminal<Output = Box<Write + Send>> + Send>),
}

/// Terminal shell we interact with
struct Shell {
    terminal: Terminal,
    config: ShellConfig,
}

impl Shell {
    /// Create a new shell
    pub fn new<T: FnMut() -> Box<Write + Send>>(mut out_fn: T, config: ShellConfig) -> Shell {
        let terminal = Shell::get_term(out_fn()).unwrap_or_else(|_| Terminal::NoColor(out_fn()));
        Shell { terminal, config }
    }

    /// Get the shell's Terminal
    fn get_term(out: Box<Write + Send>) -> term::Result<Terminal> {
        Ok(Shell::get_terminfo_term(out))
    }

    /// Get the terminfo Terminal
    fn get_terminfo_term(out: Box<Write + Send>) -> Terminal {
        match ::term::terminfo::TermInfo::from_env() {
            Ok(ti) => {
                let term = TerminfoTerminal::new_with_terminfo(out, ti);
                if term.supports_color() {
                    Terminal::Colored(Box::new(term))
                } else {
                    Terminal::NoColor(term.into_inner())
                }
            }
            Err(_) => Terminal::NoColor(out),
        }
    }

    /// Say a status message with the given color
    pub fn status<T, U>(
        &mut self,
        color: Color,
        status: T,
        message: U,
        justified: bool,
    ) -> term::Result<()>
    where
        T: fmt::Display,
        U: fmt::Display,
    {
        self.reset()?;

        if color != BLACK {
            self.fg(color)?;
        }

        if self.supports_attr(Attr::Bold) {
            self.attr(Attr::Bold)?;
        }

        if justified {
            write!(self, "{:>12}", status.to_string())?;
        } else {
            write!(self, "{}", status)?;
        }

        self.reset()?;
        writeln!(self, " {}", message)?;
        self.flush()?;

        Ok(())
    }

    fn fg(&mut self, color: Color) -> term::Result<bool> {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref mut c) if colored => c.fg(color)?,
            _ => return Ok(false),
        }

        Ok(true)
    }

    fn attr(&mut self, attr: Attr) -> term::Result<bool> {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref mut c) if colored => c.attr(attr)?,
            _ => return Ok(false),
        }

        Ok(true)
    }

    fn supports_attr(&self, attr: Attr) -> bool {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref c) if colored => c.supports_attr(attr),
            _ => false,
        }
    }

    fn reset(&mut self) -> term::Result<()> {
        let colored = self.colored();

        match self.terminal {
            Terminal::Colored(ref mut c) if colored => c.reset()?,
            _ => (),
        }

        Ok(())
    }

    fn colored(&self) -> bool {
        self.config.tty && ColorConfig::Auto == self.config.color_config
            || ColorConfig::Always == self.config.color_config
    }
}

impl Write for Shell {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.terminal {
            Terminal::Colored(ref mut c) => c.write(buf),
            Terminal::NoColor(ref mut n) => n.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.terminal {
            Terminal::Colored(ref mut c) => c.flush(),
            Terminal::NoColor(ref mut n) => n.flush(),
        }
    }
}
