//! Automatically attempt to fix vulnerable dependencies

use crate::vulnerability::Vulnerability;
use cargo_lock::{Lockfile, Package};
use std::path::PathBuf;
use std::process::Command;

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

    /// Returns a command that calls `cargo update` with the right arguments
    /// to attempt to fix this vulnerability.
    ///
    /// Note that the success of the command does not mean
    /// the vulnerability was actually fixed!
    /// It may remain if no semver-compatible fix was available.
    pub fn get_fix_command(&self, vulnerability: &Vulnerability, dry_run: bool) -> Command {
        let cargo_path = std::env::var_os("CARGO").unwrap_or("cargo".into());
        let pkg_name = &vulnerability.package.name;
        let mut command = Command::new(&cargo_path);
        command.arg("--manifest-path").arg(&self.manifest_path);
        if dry_run {
            command.arg("--dry-run");
        }
        // there can be more than one version of a given package in the lockfile, so we need to iterate over all of them
        // TODO: only consider vulnerable versions
        for pkg in self
            .lockfile
            .packages
            .iter()
            .filter(|pkg| &pkg.name == pkg_name)
        {
            let pkgid = pkgid(pkg);
            command.arg(&pkgid);
        }

        command
    }
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
