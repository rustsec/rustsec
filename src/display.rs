//! Terminal handling code
//!
//! Some portions borrowed from the Cargo project: https://github.com/rust-lang/cargo
//! These portions are redistributed under the same license as Cargo (shared by cargo-audit itself)

use libc;
use lockfile::Package;
use rustsec::advisory::Advisory;
use std::fmt;
use std::io;
use std::io::prelude::*;
use term::{self, TerminfoTerminal, Attr};
use term::Terminal as RawTerminal;
use term::color::{Color, BLACK, WHITE, RED};

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
            }
            .fmt(f)
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
        let term = match Shell::get_term(out_fn()) {
            Ok(t) => t,
            Err(_) => Terminal::NoColor(out_fn()),
        };

        Shell {
            terminal: term,
            config: config,
        }
    }

    #[cfg(any(windows))]
    fn get_term(out: Box<Write + Send>) -> term::Result<Terminal> {
        // Check if the creation of a console will succeed
        if ::term::WinConsole::new(vec![0u8; 0]).is_ok() {
            let t = ::term::WinConsole::new(out)?;
            if !t.supports_color() {
                Ok(NoColor(Box::new(t)))
            } else {
                Ok(Colored(Box::new(t)))
            }
        } else {
            // If we fail to get a windows console, we try to get a `TermInfo` one
            Ok(Shell::get_terminfo_term(out))
        }
    }

    #[cfg(any(unix))]
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

    pub fn say<T: ToString>(&mut self, message: T, color: Color) -> term::Result<()> {
        self.reset()?;

        if color != BLACK {
            self.fg(color)?;
        }

        write!(self, "{}\n", message.to_string())?;
        self.reset()?;
        self.flush()?;

        Ok(())
    }

    pub fn say_status<T, U>(&mut self,
                            status: T,
                            message: U,
                            color: Color,
                            justified: bool)
                            -> term::Result<()>
        where T: fmt::Display,
              U: fmt::Display
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
        write!(self, " {}\n", message)?;
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
        self.config.tty && ColorConfig::Auto == self.config.color_config ||
        ColorConfig::Always == self.config.color_config
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

pub fn shell(color_config: ColorConfig) -> Shell {
    let config = ShellConfig {
        color_config: color_config,
        tty: isatty(),
    };
    Shell::create(|| Box::new(io::stdout()), config)
}

pub fn not_found(shell: &mut Shell, filename: &str) -> term::Result<()> {
    shell.say_status("error:",
                    format!("Couldn't find '{}'!", filename),
                    RED,
                    false)?;
    shell.say("\nRun \"cargo build\" to generate lockfile before running audit",
             WHITE)?;

    Ok(())
}

pub fn vulns_found(shell: &mut Shell, vuln_count: usize) -> term::Result<()> {
    shell.fg(RED)?;
    shell.attr(Attr::Bold)?;

    if vuln_count == 1 {
        write!(shell, "\n1 vulnerability found!\n")?;
    } else {
        write!(shell, "\n{} vulnerabilities found!\n", vuln_count)?;
    }

    Ok(())
}

pub fn advisory(shell: &mut Shell, package: &Package, advisory: &Advisory) -> term::Result<()> {
    attribute(shell, "\nID", &advisory.id)?;
    attribute(shell, "Crate", &package.name)?;
    attribute(shell, "Version", &package.version)?;

    if let Some(ref date) = advisory.date {
        attribute(shell, "Date", date)?;
    }

    if let Some(ref url) = advisory.url {
        attribute(shell, "URL", url)?;
    }

    attribute(shell, "Title", &advisory.title)?;

    let mut fixed_versions = String::new();
    let version_count = advisory.patched_versions.len();

    for (i, version) in advisory.patched_versions.iter().enumerate() {
        fixed_versions.push_str(&version.to_string());

        if i < version_count - 1 {
            fixed_versions.push_str(", ");
        }
    }

    attribute(shell, "Solution: upgrade to", &fixed_versions)?;

    Ok(())
}

#[cfg(unix)]
#[allow(unsafe_code)]
fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
}

fn attribute(shell: &mut Shell, name: &str, value: &str) -> term::Result<()> {
    shell.attr(Attr::Bold)?;
    shell.fg(RED)?;
    write!(shell, "{}: ", name)?;

    shell.reset()?;
    shell.attr(Attr::Bold)?;
    write!(shell, "{}\n", value)?;

    shell.reset()?;

    Ok(())
}
