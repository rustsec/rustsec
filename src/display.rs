use lockfile::Package;
use rustsec::advisory::Advisory;
use term::{self, Attr, Terminal};
use term::color::{self, Color};

// TODO: Macros, cleaner API, support for disabling colors (possibly using Cargo settings?)
// Cargo's `Shell` type may be useful here
pub fn notify<T>(terminal: &mut Box<T>,
                 color: Color,
                 status: &str,
                 message: &str)
                 -> term::Result<()>
    where T: Terminal + ?Sized
{
    terminal.attr(Attr::Bold)?;
    terminal.fg(color)?;
    write!(terminal, "{:>12}", status)?;

    terminal.reset()?;
    write!(terminal, " {}\n", message)?;

    terminal.flush()?;
    Ok(())
}

pub fn not_found<T>(terminal: &mut Box<T>, filename: &str) -> term::Result<()>
    where T: Terminal + ?Sized
{
    terminal.attr(Attr::Bold)?;
    terminal.fg(color::RED)?;
    write!(terminal, "error: ")?;

    terminal.reset()?;
    terminal.attr(Attr::Bold)?;
    writeln!(terminal, "Couldn't find '{}'!", filename)?;

    terminal.reset()?;
    writeln!(terminal,
             "\nRun \"cargo build\" to generate lockfile before running audit")?;

    terminal.flush()?;
    Ok(())
}

pub fn vulns_found<T>(terminal: &mut Box<T>, vuln_count: usize) -> term::Result<()>
    where T: Terminal + ?Sized
{
    terminal.attr(Attr::Bold)?;
    terminal.fg(color::RED)?;

    if vuln_count == 1 {
        write!(terminal, "\n1 vulnerability found!\n")?;
    } else {
        write!(terminal, "\n{} vulnerabilities found!\n", vuln_count)?;
    }

    terminal.reset()?;
    terminal.flush()?;
    Ok(())
}

pub fn advisory<T>(terminal: &mut Box<T>,
                   package: &Package,
                   advisory: &Advisory)
                   -> term::Result<()>
    where T: Terminal + ?Sized
{
    write!(terminal, "\n")?;

    attribute(terminal, "ID", &advisory.id)?;
    attribute(terminal, "Crate", &package.name)?;
    attribute(terminal, "Version", &package.version)?;

    if let Some(ref date) = advisory.date {
        attribute(terminal, "Date", date)?;
    }

    if let Some(ref url) = advisory.url {
        attribute(terminal, "URL", url)?;
    }

    attribute(terminal, "Title", &advisory.title)?;

    let mut fixed_versions = String::new();
    let version_count = advisory.patched_versions.len();

    for (i, version) in advisory.patched_versions.iter().enumerate() {
        fixed_versions.push_str(&version.to_string());

        if i < version_count - 1 {
            fixed_versions.push_str(", ");
        }
    }

    attribute(terminal, "Solution: upgrade to", &fixed_versions)?;

    terminal.reset()?;
    terminal.flush()?;

    Ok(())
}

fn attribute<T>(terminal: &mut Box<T>, name: &str, value: &str) -> term::Result<()>
    where T: Terminal + ?Sized
{
    terminal.attr(Attr::Bold)?;
    terminal.fg(color::RED)?;
    write!(terminal, "{}: ", name)?;

    terminal.reset()?;
    terminal.attr(Attr::Bold)?;
    write!(terminal, "{}\n", value)?;

    terminal.reset()?;

    Ok(())
}
