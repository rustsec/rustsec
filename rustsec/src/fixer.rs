//! Automatically attempt to fix vulnerable dependencies

use crate::vulnerability::Vulnerability;
use cargo_lock::{Lockfile, Package};
use std::process::Command;
use std::{path::PathBuf, process::Output};

/// Auto-fixer for vulnerable dependencies
#[cfg_attr(docsrs, doc(cfg(feature = "fix")))]
pub struct Fixer {
    manifest_path: PathBuf,
    lockfile: Lockfile,
}

impl Fixer {
    /// Create a new [`Fixer`] for the given `Cargo.toml` file
    pub fn new(cargo_toml: PathBuf, cargo_lock: Lockfile) -> Self {
        Self {
            manifest_path: cargo_toml,
            lockfile: cargo_lock,
        }
    }

    /// Attempt to fix the given vulnerability.
    /// This function will succeed even if there is no semver-compatible fix available.
    pub fn fix(&self, vulnerability: &Vulnerability, dry_run: bool) -> FixReport {
        let mut outcomes = Vec::new();
        let cargo_path = std::env::var_os("CARGO").unwrap_or("cargo".into());
        let pkg_name = &vulnerability.package.name;
        // there can be more than one version of a given package in the lockfile, so we need to iterate over all of them
        // TODO: only consider vulnerable versions
        for pkg in self
            .lockfile
            .packages
            .iter()
            .filter(|pkg| &pkg.name == pkg_name)
        {
            let mut command = Command::new(&cargo_path);
            command.arg("--manifest-path").arg(&self.manifest_path);
            if dry_run {
                command.arg("--dry-run");
            }
            let pkgid = pkgid(pkg);
            command.arg(&pkgid);
            // Sadly `cargo update` has no JSON output so we cannot reliably know the outcome
            let output = command.output();
            outcomes.push(FixOutcome {
                package: pkg.to_owned(),
                output: output,
            })
        }

        FixReport { outcomes }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct FixReport {
    pub outcomes: Vec<FixOutcome>,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct FixOutcome {
    pub package: Package,
    /// Output of the `cargo update` command,
    /// including the stdout, stderr and exit code.
    pub output: Result<Output, std::io::Error>,
}

/// Returns a Cargo unique identifier for a package.
/// See `cargo help pkgid` for more info.
///
/// We need to pass these to `cargo update` because otherwise
/// the package specification will be ambiguous, and it will refuse to do anything.
fn pkgid(pkg: &Package) -> String {
    match pkg.source.as_ref() {
        Some(source) => format!("{}#{}@{}", source, pkg.name, pkg.version),
        None => format!("{}@{}", pkg.name, pkg.version),
    }
}
