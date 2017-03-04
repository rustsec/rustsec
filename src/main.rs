//! Audit Cargo.lock files for crates containing security vulnerabilities

#![crate_name = "cargo_audit"]
#![crate_type = "bin"]

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

mod display;
mod lockfile;

extern crate clap;
extern crate rustsec;
extern crate semver;
extern crate term;
extern crate toml;

use clap::{App, SubCommand};
use rustsec::AdvisoryDatabase;
use std::process::exit;

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

    let dependencies_result = lockfile::load(filename);

    if !dependencies_result.is_ok() {
        display::not_found(&mut stdout, &filename);
        exit(1);
    };

    let dependencies = dependencies_result.unwrap();

    display::notify(&mut stdout,
                    term::color::GREEN,
                    "Fetching",
                    &format!("advisories `{}`", url));

    let advisory_db = AdvisoryDatabase::fetch_from_url(url)
        .expect("Couldn't fetch advisory database");

    display::notify(&mut stdout,
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
            display::notify(&mut stdout,
                            term::color::RED,
                            "Warning",
                            "Vulnerable crates found!")
        }

        vuln_count += advisories.len();

        for advisory in advisories {
            display::advisory(&mut stdout, &package, advisory);
        }
    }

    if vuln_count == 0 {
        display::notify(&mut stdout,
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
