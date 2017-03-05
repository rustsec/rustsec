//! Audit Cargo.lock files for crates containing security vulnerabilities

#![crate_name = "cargo_audit"]
#![crate_type = "bin"]

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

mod display;
mod lockfile;

extern crate clap;
extern crate libc;
extern crate rustsec;
extern crate semver;
extern crate term;
extern crate toml;

use clap::{App, Arg, SubCommand};
use display::ColorConfig;
use rustsec::AdvisoryDatabase;
use std::process::exit;
use term::color::{RED, GREEN};

fn main() {
    let matches = App::new("cargo")
        .subcommand(SubCommand::with_name("audit")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Tony Arcieri <bascule@gmail.com>")
            .about("Audit Cargo.lock for crates with security vulnerabilities.")
            .arg_from_usage("-f, --file=[NAME] 'Cargo lockfile to inspect (default: Cargo.lock)'")
            .arg_from_usage("-u, --url=[URL] 'URL from which to fetch advisory database'")
            .arg(Arg::from_usage("--color=[COLOR] Colored output")
                .possible_values(&["auto", "always", "never"])))
        .get_matches();

    let (filename, url, color_config) = if let Some(audit_matches) =
        matches.subcommand_matches("audit") {
        (audit_matches.value_of("file").unwrap_or("Cargo.lock"),
         audit_matches.value_of("url").unwrap_or(rustsec::ADVISORY_DB_URL),
         audit_matches.value_of("color").unwrap_or("auto"))
    } else {
        panic!("cargo-audit is intended to be invoked as a cargo subcommand");
    };

    let mut shell = display::shell(match color_config {
        "always" => ColorConfig::Always,
        "never" => ColorConfig::Never,
        _ => ColorConfig::Auto,
    });

    let dependencies_result = lockfile::load(filename);

    if !dependencies_result.is_ok() {
        display::not_found(&mut shell, filename).unwrap();
        exit(1);
    };

    let dependencies = dependencies_result.unwrap();

    shell.say_status("Fetching", &format!("advisories `{}`", url), GREEN, true).unwrap();

    let advisory_db = AdvisoryDatabase::fetch_from_url(url)
        .expect("Couldn't fetch advisory database");

    shell.say_status("Scanning",
                    &format!("{} crates for vulnerabilities ({} advisories in database)",
                             dependencies.len(),
                             advisory_db.iter().len()),
                    GREEN,
                    true)
        .unwrap();

    let mut vuln_count: usize = 0;

    for package in dependencies {
        let advisories = advisory_db.find_vulns_for_crate(&package.name, &package.version)
            .expect("error obtaining advisories for this crate");

        if vuln_count == 0 && !advisories.is_empty() {
            shell.say_status("Warning", "Vulnerable crates found!", RED, true)
                .unwrap()
        }

        vuln_count += advisories.len();

        for advisory in advisories {
            display::advisory(&mut shell, &package, advisory).unwrap();
        }
    }

    if vuln_count == 0 {
        shell.say_status("Success", "No vulnerable packages found", GREEN, true)
            .unwrap();

        exit(0);
    } else {
        display::vulns_found(&mut shell, vuln_count).unwrap();
        exit(1);
    }
}
