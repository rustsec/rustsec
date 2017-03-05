//! Audit Cargo.lock files for crates containing security vulnerabilities

#![crate_name = "cargo_audit"]
#![crate_type = "bin"]

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

mod lockfile;
mod shell;

extern crate clap;
extern crate libc;
extern crate rustsec;
extern crate semver;
extern crate term;
extern crate toml;

use clap::{App, Arg, SubCommand};
use lockfile::Package;
use rustsec::AdvisoryDatabase;
use rustsec::advisory::Advisory;
use shell::{ColorConfig, Shell};
use std::process::exit;
use term::color::{RED, GREEN, WHITE};

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

    let mut shell = shell::create(match color_config {
        "always" => ColorConfig::Always,
        "never" => ColorConfig::Never,
        _ => ColorConfig::Auto,
    });

    let dependencies_result = lockfile::load(filename);

    if !dependencies_result.is_ok() {
        not_found(&mut shell, filename).unwrap();
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
            display_advisory(&mut shell, &package, advisory).unwrap();
        }
    }

    if vuln_count == 0 {
        shell.say_status("Success", "No vulnerable packages found", GREEN, true)
            .unwrap();

        exit(0);
    } else {
        vulns_found(&mut shell, vuln_count).unwrap();
        exit(1);
    }
}

fn not_found(shell: &mut Shell, filename: &str) -> term::Result<()> {
    shell.say_status("error:",
                    format!("Couldn't find '{}'!", filename),
                    RED,
                    false)?;
    shell.say("\nRun \"cargo build\" to generate lockfile before running audit",
             WHITE)?;

    Ok(())
}

fn vulns_found(shell: &mut Shell, vuln_count: usize) -> term::Result<()> {
    if vuln_count == 1 {
        shell.say_status("\nerror:", "1 vulnerability found!", RED, false)?;
    } else {
        shell.say_status("\nerror:",
                        format!("{} vulnerabilities found!", vuln_count),
                        RED,
                        false)?;
    }

    Ok(())
}

fn display_advisory(shell: &mut Shell, package: &Package, advisory: &Advisory) -> term::Result<()> {
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

fn attribute(shell: &mut Shell, name: &str, value: &str) -> term::Result<()> {
    shell.say_status(format!("{}:", name), value, RED, false)
}
