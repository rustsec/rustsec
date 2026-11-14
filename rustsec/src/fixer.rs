//! Automatically attempt to fix vulnerable dependencies
//!
//! This module is **experimental**, and its behavior may change in the future.

use crate::{Error, ErrorKind, vulnerability::Vulnerability};
use cargo_lock::{Lockfile, Package, package::Name};
use semver::{Comparator, Op, Version, VersionReq};
use std::path::{Path, PathBuf};
use std::process::Command;

/// A published release of a crate, as recorded in a registry index
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    /// Version of this release
    pub version: Version,
    /// Whether this release has been yanked
    pub yanked: bool,
    /// Normal and build dependencies of this release, as `(package name, requirement)`.
    ///
    /// The name must be the real package name, not the alias of a renamed
    /// dependency (`package = "..."` in the index entry). Dev-dependencies
    /// must be left out, as they never appear in a dependent's lockfile.
    pub dependencies: Vec<(Name, VersionReq)>,
}

/// Source of registry release information used by [`Fixer::plan_fix_via_dependents`]
///
/// The planner asks for the same crate several times while walking the
/// lockfile graph, so implementations backed by the network or disk should
/// cache their results.
pub trait ReleaseIndex {
    /// All published releases of the crate `name`.
    ///
    /// A crate missing from the index is an error, not an empty list: the
    /// planner only asks about crates in the lockfile, so a missing one means
    /// the index is incomplete or stale.
    fn releases(&mut self, name: &Name) -> Result<Vec<Release>, Error>;
}

/// Outcome of [`Fixer::plan_fix_via_dependents`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DependentFix {
    /// Run `cargo update <package> --precise <version>`
    Update {
        /// Locked package to move
        package: Box<Package>,
        /// Version to move it to
        version: Version,
    },
    /// No dependent has published a semver-compatible release that admits a patched version
    NoReleasedFix,
    /// A workspace member, path, git or alternate-registry package would have to change its own
    /// `Cargo.toml` requirement; a lockfile-only fix is not possible
    ManifestChangeRequired(Box<Package>),
    /// More than one dependent would have to move; this is not attempted
    MultipleBlockingDependents(Vec<Package>),
}

/// Auto-fixer for vulnerable dependencies
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
        let pkg_name = &vulnerability.package.name;
        let mut command = self.base_update_command(dry_run);
        // There can be more than one version of a given package in the
        // lockfile, so we need to iterate over all of them.
        //
        // Before we do, we need to add -- so that a lockfile entry starting
        // with -- isn't interpreted as a flag by cargo.
        command.arg("--");
        for pkg in self.lockfile.packages.iter().filter(|pkg| {
            &pkg.name == pkg_name && vulnerability.versions.is_vulnerable(&pkg.version)
        }) {
            let pkgid = pkgid(pkg);
            command.arg(&pkgid);
        }

        command
    }

    /// Plan a lockfile-only fix for `vulnerability` that moves the vulnerable
    /// package or, when its dependents pin it, one of its dependents.
    ///
    /// Starting from the patched releases of the vulnerable package, this walks
    /// up the lockfile graph. A dependent whose locked release already admits a
    /// candidate narrows the candidates; a single dependent that admits none of
    /// them has to move itself, so the walk continues with those of its
    /// semver-compatible newer releases that require a candidate and exclude
    /// the currently locked version. The lowest qualifying version is chosen to
    /// keep lockfile churn small.
    ///
    /// Requirements of workspace members, path, git and alternate-registry
    /// packages are not in the index; for those the default caret requirement on the locked version is
    /// assumed, and `cargo update --precise` enforces the real one.
    ///
    /// Known limitations: only one dependent per level may need to move, and a
    /// dependent release that drops the vulnerable dependency altogether is not
    /// considered a fix.
    pub fn plan_fix_via_dependents(
        &self,
        vulnerability: &Vulnerability,
        index: &mut dyn ReleaseIndex,
    ) -> Result<DependentFix, Error> {
        let candidates = patched_releases(vulnerability, index)?;
        self.plan_move(&vulnerability.package, candidates, index)
    }

    /// Returns a `cargo update --precise` command applying `fix`,
    /// or `None` if `fix` does not call for an update.
    pub fn get_fix_via_dependents_command(
        &self,
        fix: &DependentFix,
        dry_run: bool,
    ) -> Option<Command> {
        let DependentFix::Update { package, version } = fix else {
            return None;
        };
        let mut command = self.base_update_command(dry_run);
        // As in `get_fix_command`, `--` keeps a lockfile entry starting with
        // `--` from being read as a flag, so every option goes before it
        command
            .arg("--precise")
            .arg(version.to_string())
            .arg("--")
            .arg(pkgid(package));
        Some(command)
    }

    /// Plan how to move the locked `package` to one of `candidates`
    fn plan_move(
        &self,
        package: &Package,
        candidates: Vec<Version>,
        index: &mut dyn ReleaseIndex,
    ) -> Result<DependentFix, Error> {
        if candidates.is_empty() {
            return Ok(DependentFix::NoReleasedFix);
        }
        let (candidates, blockers) =
            self.narrow_candidates_by_dependents(package, candidates, index)?;
        match blockers.as_slice() {
            [] => Ok(update_to_lowest(package, &candidates)),
            [blocker] if is_crates_io_package(blocker) => {
                let blocker_candidates =
                    releases_requiring_move(blocker, package, &candidates, index)?;
                self.plan_move(blocker, blocker_candidates, index)
            }
            [member] => Ok(DependentFix::ManifestChangeRequired(Box::new(
                (*member).clone(),
            ))),
            _ => Ok(DependentFix::MultipleBlockingDependents(
                blockers.into_iter().cloned().collect(),
            )),
        }
    }

    /// Keep the candidates every dependent of `package` admits, and collect
    /// the dependents that admit none of them.
    fn narrow_candidates_by_dependents(
        &self,
        package: &Package,
        mut candidates: Vec<Version>,
        index: &mut dyn ReleaseIndex,
    ) -> Result<(Vec<Version>, Vec<&Package>), Error> {
        let mut blockers = Vec::new();
        for dependent in self.dependents_of(package) {
            let admitted = admitted_by_dependent(dependent, package, &candidates, index)?;
            if admitted.is_empty() {
                blockers.push(dependent);
            } else {
                candidates = admitted;
            }
        }
        Ok((candidates, blockers))
    }

    /// Packages in the lockfile that depend on exactly this `package`
    fn dependents_of(&self, package: &Package) -> Vec<&Package> {
        self.lockfile
            .packages
            .iter()
            .filter(|pkg| pkg.dependencies.iter().any(|dep| dep.matches(package)))
            .collect()
    }

    /// `cargo update` with the manifest path and dry-run flag applied
    fn base_update_command(&self, dry_run: bool) -> Command {
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
        command
    }
}

/// Released, non-yanked, stable versions of the vulnerable package that are
/// newer than the locked one and not vulnerable
fn patched_releases(
    vulnerability: &Vulnerability,
    index: &mut dyn ReleaseIndex,
) -> Result<Vec<Version>, Error> {
    let releases = index.releases(&vulnerability.package.name)?;
    Ok(releases
        .into_iter()
        .filter(is_installable)
        .map(|release| release.version)
        .filter(|version| *version > vulnerability.package.version)
        .filter(|version| !vulnerability.versions.is_vulnerable(version))
        .collect())
}

/// The `candidates` for `package` that the locked `dependent` admits
fn admitted_by_dependent(
    dependent: &Package,
    package: &Package,
    candidates: &[Version],
    index: &mut dyn ReleaseIndex,
) -> Result<Vec<Version>, Error> {
    let requirements = if is_crates_io_package(dependent) {
        let locked = locked_release(dependent, index)?;
        requirements_on(&locked, &package.name)
    } else {
        vec![caret_requirement(&package.version)]
    };
    Ok(candidates
        .iter()
        .filter(|version| requirements.iter().any(|req| req.matches(version)))
        .cloned()
        .collect())
}

/// Semver-compatible newer releases of `dependent` whose requirement on
/// `package` admits one of `candidates` but no longer admits its locked version
fn releases_requiring_move(
    dependent: &Package,
    package: &Package,
    candidates: &[Version],
    index: &mut dyn ReleaseIndex,
) -> Result<Vec<Version>, Error> {
    let compatible = caret_requirement(&dependent.version);
    let releases = index.releases(&dependent.name)?;
    Ok(releases
        .into_iter()
        .filter(is_installable)
        .filter(|release| {
            release.version > dependent.version && compatible.matches(&release.version)
        })
        .filter(|release| forces_move_to_candidate(release, package, candidates))
        .map(|release| release.version)
        .collect())
}

/// Does `release` require one of `candidates` for `package` and exclude its locked version?
fn forces_move_to_candidate(release: &Release, package: &Package, candidates: &[Version]) -> bool {
    let requirements = requirements_on(release, &package.name);
    let admits_candidate = candidates
        .iter()
        .any(|version| requirements.iter().any(|req| req.matches(version)));
    let admits_locked = requirements.iter().any(|req| req.matches(&package.version));
    admits_candidate && !admits_locked
}

/// The index entry for the locked version of `package`
fn locked_release(package: &Package, index: &mut dyn ReleaseIndex) -> Result<Release, Error> {
    index
        .releases(&package.name)?
        .into_iter()
        .find(|release| release.version == package.version)
        .ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                format!(
                    "No such version in registry index: {} {}",
                    package.name, package.version
                ),
            )
        })
}

/// Requirements `release` places on the package called `name`
fn requirements_on(release: &Release, name: &Name) -> Vec<VersionReq> {
    release
        .dependencies
        .iter()
        .filter(|(dep, _)| dep == name)
        .map(|(_, req)| req.clone())
        .collect()
}

/// Update `package` to the lowest of `candidates`, if there is one
fn update_to_lowest(package: &Package, candidates: &[Version]) -> DependentFix {
    match candidates.iter().min() {
        Some(version) => DependentFix::Update {
            package: Box::new(package.clone()),
            version: version.clone(),
        },
        None => DependentFix::NoReleasedFix,
    }
}

/// Would `cargo update` ever select this release?
fn is_installable(release: &Release) -> bool {
    !release.yanked && release.version.pre.is_empty()
}

/// Only crates.io packages can be looked up in the [`ReleaseIndex`];
/// alternate registries are treated like path and git packages
fn is_crates_io_package(package: &Package) -> bool {
    package
        .source
        .as_ref()
        .is_some_and(|source| source.is_default_registry())
}

/// The requirement Cargo writes for `version` by default, i.e. `^version`
fn caret_requirement(version: &Version) -> VersionReq {
    VersionReq {
        comparators: vec![Comparator {
            op: Op::Caret,
            major: version.major,
            minor: Some(version.minor),
            patch: Some(version.patch),
            pre: version.pre.clone(),
        }],
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Advisory;

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
