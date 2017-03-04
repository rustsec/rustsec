use lockfile::Package;
use rustsec::advisory::Advisory;
use term::Attr;
use term::Terminal;
use term::color::{self, Color};

// TODO: Macros, cleaner API, support for disabling colors (possibly using Cargo settings?)
// Cargo's `Shell` type may be useful here
pub fn notify<T>(terminal: &mut Box<T>, color: Color, status: &str, message: &str)
    where T: Terminal + ?Sized
{
    terminal.attr(Attr::Bold).unwrap();
    terminal.fg(color).unwrap();
    write!(terminal, "{:>12}", status).unwrap();

    terminal.reset().unwrap();
    write!(terminal, " {}\n", message).unwrap();
}

pub fn not_found<T>(terminal: &mut Box<T>, filename: &str)
    where T: Terminal + ?Sized
{
    terminal.attr(Attr::Bold).unwrap();
    terminal.fg(color::RED).unwrap();
    write!(terminal, "error: ").unwrap();

    terminal.reset().unwrap();
    terminal.attr(Attr::Bold).unwrap();
    writeln!(terminal, "Couldn't find '{}'!", filename).unwrap();

    terminal.reset().unwrap();
    writeln!(terminal,
             "\nRun \"cargo build\" to generate lockfile before running audit")
        .unwrap();
}

pub fn advisory<T>(terminal: &mut Box<T>, package: &Package, advisory: &Advisory)
    where T: Terminal + ?Sized
{
    write!(terminal, "\n").unwrap();

    attribute(terminal, "ID", &advisory.id);
    attribute(terminal, "Crate", &package.name);
    attribute(terminal, "Version", &package.version);

    if let Some(ref date) = advisory.date {
        attribute(terminal, "Date", date);
    }

    if let Some(ref url) = advisory.url {
        attribute(terminal, "URL", url);
    }

    attribute(terminal, "Title", &advisory.title);

    let mut fixed_versions = String::new();
    let version_count = advisory.patched_versions.len();

    for (i, version) in advisory.patched_versions.iter().enumerate() {
        fixed_versions.push_str(&version.to_string());

        if i < version_count - 1 {
            fixed_versions.push_str(", ");
        }
    }

    attribute(terminal, "Solution: upgrade to", &fixed_versions);

    terminal.reset().unwrap();
}

fn attribute<T>(terminal: &mut Box<T>, name: &str, value: &str)
    where T: Terminal + ?Sized
{
    terminal.attr(Attr::Bold).unwrap();
    terminal.fg(color::RED).unwrap();
    write!(terminal, "{}: ", name).unwrap();

    terminal.reset().unwrap();
    terminal.attr(Attr::Bold).unwrap();
    write!(terminal, "{}\n", value).unwrap();

    terminal.reset().unwrap();
}
