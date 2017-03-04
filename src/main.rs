//! Audit Cargo.lock files for crates containing security vulnerabilities

#![crate_name = "cargo_audit"]
#![crate_type = "bin"]

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

extern crate clap;
extern crate rustsec;
extern crate semver;
extern crate term;
extern crate toml;

use clap::{App, SubCommand};
use rustsec::AdvisoryDatabase;
use rustsec::advisory::Advisory;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

// TODO: Use serde
#[derive(Debug, PartialEq)]
struct Package {
    name: String,
    version: String,
}

fn load_lockfile(filename: &str) -> Result<Vec<Package>, std::io::Error> {
    let path = Path::new(filename);
    let mut file = File::open(&path)?;
    let mut body = String::new();

    file.read_to_string(&mut body).expect("Error reading lockfile!");

    let toml = body.parse::<toml::Value>().expect("Couldn't parse the lockfile!");
    let packages = match toml["package"] {
        toml::Value::Array(ref arr) => arr,
        _ => panic!("lockfile is malformed!"),
    };

    Ok(packages.iter()
        .map(|package| {
            Package {
                name: String::from(package["name"].as_str().expect("missing package name!")),
                version: String::from(package["version"]
                    .as_str()
                    .expect("missing package version!")),
            }
        })
        .collect())
}

// TODO: Macros, cleaner API, support for disabling colors (possibly using Cargo settings?)
// Cargo's `Shell` type may be useful here
fn notify<T>(terminal: &mut Box<T>, color: term::color::Color, status: &str, message: &str)
    where T: term::Terminal + ?Sized
{
    terminal.attr(term::Attr::Bold).unwrap();
    terminal.fg(color).unwrap();
    write!(terminal, "{:>12}", status).unwrap();

    terminal.reset().unwrap();
    write!(terminal, " {}\n", message).unwrap();
}

fn display_attribute<T>(terminal: &mut Box<T>, name: &str, value: &str)
    where T: term::Terminal + ?Sized
{
    terminal.attr(term::Attr::Bold).unwrap();
    terminal.fg(term::color::RED).unwrap();
    write!(terminal, "{}: ", name).unwrap();

    terminal.reset().unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    write!(terminal, "{}\n", value).unwrap();

    terminal.reset().unwrap();
}

fn display_advisory<T>(terminal: &mut Box<T>, package: &Package, advisory: &Advisory)
    where T: term::Terminal + ?Sized
{
    write!(terminal, "\n").unwrap();

    display_attribute(terminal, "ID", &advisory.id);
    display_attribute(terminal, "Crate", &package.name);
    display_attribute(terminal, "Version", &package.version);

    if let Some(ref date) = advisory.date {
        display_attribute(terminal, "Date", date);
    }

    if let Some(ref url) = advisory.url {
        display_attribute(terminal, "URL", url);
    }

    display_attribute(terminal, "Title", &advisory.title);

    let mut fixed_versions = String::new();
    let version_count = advisory.patched_versions.len();

    for (i, version) in advisory.patched_versions.iter().enumerate() {
        fixed_versions.push_str(&version.to_string());

        if i < version_count - 1 {
            fixed_versions.push_str(", ");
        }
    }

    display_attribute(terminal, "Solution: upgrade to", &fixed_versions);

    terminal.reset().unwrap();
}

fn main() {
    let mut stdout = term::stdout().unwrap();

    let matches = App::new("cargo")
        .subcommand(SubCommand::with_name("audit")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Tony Arcieri <bascule@gmail.com>")
            .about("Audit Cargo.lock for crates with security vulnerabilities.")
            .arg_from_usage("-f, --file=[NAME] 'Cargo lockfile to inspect (default: Cargo.lock)'")
            .arg_from_usage("-u, --url=[URL] 'URL from which to fetch advisory database'"))
        .get_matches();

    let (filename, url) = if let Some(audit_matches) = matches.subcommand_matches("audit") {
        (audit_matches.value_of("file").unwrap_or("Cargo.lock"),
         audit_matches.value_of("url").unwrap_or(rustsec::ADVISORY_DB_URL))
    } else {
        panic!("cargo-audit is intended to be invoked as a cargo subcommand");
    };

    let dependencies_result = load_lockfile(filename);

    if !dependencies_result.is_ok() {
        stdout.attr(term::Attr::Bold).unwrap();
        stdout.fg(term::color::RED).unwrap();
        write!(stdout, "error: ").unwrap();

        stdout.reset().unwrap();
        stdout.attr(term::Attr::Bold).unwrap();
        writeln!(stdout, "Couldn't find '{}'!", filename).unwrap();

        stdout.reset().unwrap();
        writeln!(stdout,
                 "\nRun \"cargo build\" to generate lockfile before running audit")
            .unwrap();

        exit(1);
    };

    let dependencies = dependencies_result.unwrap();

    notify(&mut stdout,
           term::color::GREEN,
           "Fetching",
           &format!("advisories `{}`", url));

    let advisory_db = AdvisoryDatabase::fetch_from_url(url)
        .expect("Couldn't fetch advisory database");

    notify(&mut stdout,
           term::color::GREEN,
           "Scanning",
           &format!("{} crates for vulnerabilities ({} advisories in database)",
                    dependencies.len(),
                    advisory_db.iter().len()));

    let mut vuln_count: usize = 0;

    for package in dependencies {
        let advisories = advisory_db.find_vulns_for_crate(&package.name, &package.version)
            .expect("error obtaining advisories for this crate");

        if vuln_count == 0 && !advisories.is_empty() {
            notify(&mut stdout,
                   term::color::RED,
                   "Warning",
                   "Vulnerable crates found!")
        }

        vuln_count += advisories.len();

        for advisory in advisories {
            display_advisory(&mut stdout, &package, advisory);
        }
    }

    if vuln_count == 0 {
        notify(&mut stdout,
               term::color::GREEN,
               "Success",
               "No vulnerable packages found");

        exit(0);
    } else {
        stdout.attr(term::Attr::Bold).unwrap();
        stdout.fg(term::color::RED).unwrap();

        if vuln_count == 1 {
            write!(stdout, "\n1 vulnerability found!\n").unwrap();
        } else {
            write!(stdout, "\n{} vulnerabilities found!\n", vuln_count).unwrap();
        }

        stdout.reset().unwrap();
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_cargo_lockfile() {
        ::load_lockfile("Cargo.lock");
    }
}
