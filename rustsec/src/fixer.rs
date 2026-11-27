//! Build `cargo update` commands for dependencies affected by advisories.
//!
//! This module is **experimental**, and its behavior may change in the future.

use crate::{advisory::Versions, vulnerability::Vulnerability};
use cargo_lock::{Lockfile, Package, SourceId};
use std::cell::OnceCell;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Builds update commands for affected dependencies.
#[cfg_attr(docsrs, doc(cfg(feature = "fix")))]
pub struct Fixer {
    lockfile: Lockfile,
    manifest_path: Option<PathBuf>,
    path_to_cargo: Option<PathBuf>,
}

impl Fixer {
    /// Create a new [`Fixer`] for the given `Cargo.lock` file
    ///
    /// `path_to_cargo` defaults to `cargo`, resolved in your `$PATH`.
    ///
    /// If the path to `Cargo.toml` is not specified, the `cargo update` command
    /// will be run in the directory with the `Cargo.lock` file.
    /// Leaving it blank will fix the entire workspace.
    pub fn new(
        cargo_lock: Lockfile,
        cargo_toml: Option<PathBuf>,
        path_to_cargo: Option<PathBuf>,
    ) -> Self {
        Self {
            lockfile: cargo_lock,
            manifest_path: cargo_toml,
            path_to_cargo,
        }
    }

    /// Returns a command that calls `cargo update` with the right arguments
    /// to attempt to fix this vulnerability.
    ///
    /// Note that the success of the command does not mean
    /// the vulnerability was actually fixed!
    /// It may remain if no semver-compatible fix was available.
    pub fn get_fix_command(&self, vulnerability: &Vulnerability, dry_run: bool) -> Command {
        let mut command = self.update_command(dry_run);
        let packages = self.lockfile.packages.iter().filter(|pkg| {
            pkg.name == vulnerability.package.name
                && vulnerability.versions.is_vulnerable(&pkg.version)
        });
        command.args(packages.map(pkgid));
        command
    }

    /// Builds a `cargo update` command for affected versions from the package's source.
    /// Returns `None` when no locked version matches.
    pub fn fix_command(
        &self,
        package: &Package,
        versions: &Versions,
        dry_run: bool,
    ) -> Option<Command> {
        let ranges = OnceCell::new();
        let mut packages = self.lockfile.packages.iter().filter(|pkg| {
            pkg.name == package.name
                && same_source(pkg.source.as_ref(), package.source.as_ref())
                && ranges
                    .get_or_init(|| crate::osv::ranges_for_advisory(versions))
                    .iter()
                    .any(|range| range.affects(&pkg.version))
        });
        let first = packages.next()?;
        let mut command = self.update_command(dry_run);
        command.arg(pkgid(first)).args(packages.map(pkgid));
        Some(command)
    }

    fn update_command(&self, dry_run: bool) -> Command {
        let cargo_path = self
            .path_to_cargo
            .as_deref()
            .unwrap_or_else(|| Path::new("cargo"));

        let mut command = Command::new(cargo_path);
        command.arg("update");
        if let Some(path) = self.manifest_path.as_ref() {
            command.arg("--manifest-path").arg(path);
        }
        if dry_run {
            command.arg("--dry-run");
        }
        // Treat all lockfile package IDs as positional arguments.
        command.arg("--");
        command
    }
}

fn same_source(a: Option<&SourceId>, b: Option<&SourceId>) -> bool {
    match (a, b) {
        // Compare the Git reference without the locked commit.
        (Some(a), Some(b)) => a.kind() == b.kind() && a.url() == b.url(),
        (None, None) => true,
        _ => false,
    }
}

/// Returns a Cargo unique identifier for a package.
/// See `cargo help pkgid` for more info.
///
/// We need to pass these to `cargo update` because otherwise
/// the package specification will be ambiguous, and it will refuse to do anything.
fn pkgid(pkg: &Package) -> String {
    let Some(source) = &pkg.source else {
        return format!("{}@{}", pkg.name, pkg.version);
    };
    if let Some(reference) = source.git_reference() {
        // The fragment in a Cargo package ID names the package and version.
        let url = source.url();
        return match reference.pretty_ref(true) {
            Some(reference) => format!("git+{url}?{reference}#{}@{}", pkg.name, pkg.version),
            None => format!("git+{url}#{}@{}", pkg.name, pkg.version),
        };
    }
    format!("{}#{}@{}", source, pkg.name, pkg.version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Advisory;

    #[test]
    fn fix_command_targets_affected_versions_from_the_same_source() {
        let registry = include_str!("../tests/support/cratesio_cargo.lock");
        let local = include_str!("../tests/support/local_cargo.lock");
        let mut lock: Lockfile = registry.parse().unwrap();
        let mut other_registry: Lockfile = local.parse().unwrap();
        other_registry.packages[0].source =
            Some("registry+https://example.org/index".parse().unwrap());
        let packages = &mut lock.packages;
        let base64 = packages.iter().find(|pkg| pkg.name.as_str() == "base64");
        let package = base64.unwrap().clone();
        packages.extend(["1.0.0", "2.0.0"].map(|version| Package {
            version: version.parse().unwrap(),
            ..package.clone()
        }));
        packages.extend(other_registry.packages.iter().cloned());

        let versions = Versions::new(vec![">=2.0.0".parse().unwrap()], vec![]).unwrap();
        let fixer = Fixer::new(lock, None, None);
        let command = fixer.fix_command(&package, &versions, false).unwrap();
        let args = command.get_args().skip_while(|arg| *arg != "--").skip(1);
        let mut targets = args.map(|arg| arg.to_str().unwrap()).collect::<Vec<_>>();
        targets.sort_unstable();
        let source = package.source.as_ref().unwrap();
        let mut expected =
            [package.version.to_string(), "1.0.0".into()].map(|v| format!("{source}#base64@{v}"));
        expected.sort_unstable();
        assert_eq!(targets, expected);
        let fixer = Fixer::new(other_registry, None, None);
        assert!(fixer.fix_command(&package, &versions, false).is_none());
    }

    #[test]
    fn fix_command_matches_git_references_across_commits() {
        let url = "git+https://example.org/fixture";
        let branch = "?branch=feature%2Ftest%23tag";
        let commit = "1234567890abcdef";
        for (reported, locked, revision, matches) in [
            ("", "", commit, true),
            (branch, branch, commit, true),
            (branch, "?branch=other", commit, false),
            (branch, branch, "fedcba0987654321", true),
        ] {
            let lockfile: Lockfile = format!(
                "version = 3\n[[package]]\nname = \"fixture\"\nversion = \"1.0.0\"\nsource = \"{url}{locked}#{revision}\""
            )
            .parse()
            .unwrap();
            let package = Package {
                source: Some(format!("{url}{reported}#{commit}").parse().unwrap()),
                ..lockfile.packages[0].clone()
            };
            let command =
                Fixer::new(lockfile, None, None).fix_command(&package, &Versions::default(), true);
            let target = command.as_ref().and_then(|cmd| cmd.get_args().last());
            let expected = format!("{url}{reported}#fixture@1.0.0");
            assert_eq!(
                target,
                matches.then_some(expected.as_ref()),
                "{reported} vs {locked}#{revision}"
            );
        }
    }

    #[test]
    fn fix_command_package_separator() {
        // Construct a lockfile with a name that could be interpreted as a flag.
        let package_name = "--config=/tmp/attacker-config";
        let lockfile: Lockfile = format!(
            r#"version = 3

[[package]]
name = "{package_name}"
version = "1.0.0"
"#
        )
        .parse()
        .expect("parse lock file");

        // Build an advisory and vulnerability with the same name.
        let advisory: Advisory = format!(
            r#"```toml
[advisory]
id = "RUSTSEC-2026-0001"
package = "{package_name}"
date = "2026-11-10"

[versions]
patched = [">=2.0.0"]
```

# Test advisory

Test.
"#
        )
        .parse()
        .expect("parse advisory");

        let vulnerability = Vulnerability::new(&advisory, &lockfile.packages[0]);

        // Build the fix command and validate that it looks as expected; that
        // is, with the -- before the package name.
        let args = Fixer::new(lockfile, Some("Cargo.toml".into()), None)
            .get_fix_command(&vulnerability, true)
            .get_args()
            .map(|arg| arg.to_str().expect("arg is valid str").to_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            args,
            [
                "update",
                "--manifest-path",
                "Cargo.toml",
                "--dry-run",
                "--",
                "--config=/tmp/attacker-config@1.0.0",
            ]
        );
    }
}
