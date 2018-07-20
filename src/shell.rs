//! Terminal handling code
//!
//! Some portions borrowed from the Cargo project: https://github.com/rust-lang/cargo
//! These portions are redistributed under the same license as Cargo (shared by cargo-audit itself)

use isatty::stdout_isatty;
use std::fmt;
use std::io;
use std::io::prelude::*;
use term::color::{Color, BLACK};
use term::Terminal as RawTerminal;
use term::{self, Attr, TerminfoTerminal};

pub fn create(color_config: ColorConfig) -> Shell {
    let tty = stdout_isatty();
    let config = ShellConfig { color_config, tty };
    Shell::create(|| Box::new(io::stdout()), config)
}

#[derive(Clone, Copy, PartialEq)]
pub enum ColorConfig {
    Auto,
    Always,
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

#[derive(Clone, Copy)]
pub struct ShellConfig {
    pub color_config: ColorConfig,
    pub tty: bool,
}

enum Terminal {
    NoColor(Box<Write + Send>),
    Colored(Box<RawTerminal<Output = Box<Write + Send>> + Send>),
}

pub struct Shell {
    terminal: Terminal,
    config: ShellConfig,
}

impl Shell {
    pub fn create<T: FnMut() -> Box<Write + Send>>(mut out_fn: T, config: ShellConfig) -> Shell {
        let terminal = match Shell::get_term(out_fn()) {
            Ok(t) => t,
            Err(_) => Terminal::NoColor(out_fn()),
        };

        Shell { terminal, config }
    }

    #[cfg(windows)]
    fn get_term(out: Box<Write + Send>) -> term::Result<Terminal> {
        // Check if the creation of a console will succeed
        if ::term::WinConsole::new(vec![0u8; 0]).is_ok() {
            let t = ::term::WinConsole::new(out)?;
            if !t.supports_color() {
                Ok(Terminal::NoColor(Box::new(t)))
            } else {
                Ok(Terminal::Colored(Box::new(t)))
            }
        } else {
            // If we fail to get a windows console, we try to get a `TermInfo` one
            Ok(Shell::get_terminfo_term(out))
        }
    }

    #[cfg(not(windows))]
    fn get_term(out: Box<Write + Send>) -> term::Result<Terminal> {
        Ok(Shell::get_terminfo_term(out))
    }

    fn get_terminfo_term(out: Box<Write + Send>) -> Terminal {
        // Use `TermInfo::from_env()` and `TerminfoTerminal::supports_color()`
        // to determine if creation of a TerminfoTerminal is possible regardless
        // of the tty status. --color options are parsed after Shell creation so
        // always try to create a terminal that supports color output. Fall back
        // to a no-color terminal regardless of whether or not a tty is present
        // and if color output is not possible.
        match ::term::terminfo::TermInfo::from_env() {
            Ok(ti) => {
                let term = TerminfoTerminal::new_with_terminfo(out, ti);
                if !term.supports_color() {
                    Terminal::NoColor(term.into_inner())
                } else {
                    // Color output is possible.
                    Terminal::Colored(Box::new(term))
                }
            }
            Err(_) => Terminal::NoColor(out),
        }
    }

    pub fn say<T: AsRef<str>>(&mut self, message: T, color: Color) -> term::Result<()> {
        self.reset()?;

        if color != BLACK {
            self.fg(color)?;
        }

        writeln!(self, "{}", message.as_ref())?;
        self.reset()?;
        self.flush()?;

        Ok(())
    }

    pub fn say_status<T, U>(
        &mut self,
        status: T,
        message: U,
        color: Color,
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
