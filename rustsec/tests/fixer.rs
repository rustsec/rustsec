//! Tests for fixing a vulnerability by updating a dependent of the vulnerable crate.
//!
//! The lockfiles and registry index used here are synthetic; they mirror the
//! `zip` <- `libduckdb-sys` <- `duckdb` chain from rustsec/rustsec#1730.

use rustsec::{
    Advisory, Error, ErrorKind, Fixer, Lockfile, Vulnerability,
    fixer::{DependentFix, Release, ReleaseIndex},
    package::{Name, Package},
};
use std::{collections::HashMap, ffi::OsStr, path::PathBuf, process::Command, str::FromStr};

/// In-memory stand-in for the crates.io index
#[derive(Default)]
struct FakeIndex {
    releases: HashMap<String, Vec<Release>>,
}

impl FakeIndex {
    /// Register a release of `name` whose dependencies are given as `(name, requirement)` pairs
    fn publish(mut self, name: &str, version: &str, dependencies: &[(&str, &str)]) -> Self {
        let release = Release {
            version: version.parse().unwrap(),
            yanked: false,
            dependencies: dependencies
                .iter()
                .map(|(dep, req)| (dep.parse().unwrap(), req.parse().unwrap()))
                .collect(),
        };
        self.releases
            .entry(name.to_owned())
            .or_default()
            .push(release);
        self
    }

    /// Mark the most recently published release of `name` as yanked
    fn yank_latest(mut self, name: &str) -> Self {
        let release = self.releases.get_mut(name).unwrap().last_mut().unwrap();
        release.yanked = true;
        self
    }
}

impl ReleaseIndex for FakeIndex {
    fn releases(&mut self, name: &Name) -> Result<Vec<Release>, Error> {
        Ok(self
            .releases
            .get(name.as_str())
            .cloned()
            .unwrap_or_default())
    }
}

const REGISTRY: &str = "registry+https://github.com/rust-lang/crates.io-index";

/// Build a lockfile whose packages are given as `(name, version, dependencies)`.
/// The package named `app` is a workspace member; all others come from crates.io.
fn lockfile(packages: &[(&str, &str, &[&str])]) -> Lockfile {
    let body: String = packages.iter().map(lockfile_package_entry).collect();
    Lockfile::from_str(&format!("version = 4\n{body}")).unwrap()
}

fn lockfile_package_entry((name, version, dependencies): &(&str, &str, &[&str])) -> String {
    let source = match *name {
        "app" => String::new(),
        _ => format!("source = \"{REGISTRY}\"\n"),
    };
    let dependencies: Vec<String> = dependencies.iter().map(|d| format!("\"{d}\"")).collect();
    format!(
        "[[package]]\nname = \"{name}\"\nversion = \"{version}\"\n{source}dependencies = [{}]\n",
        dependencies.join(", ")
    )
}

/// A vulnerability in `package_name` that is patched in `patched`
fn vulnerability(lockfile: &Lockfile, package_name: &str, patched: &str) -> Vulnerability {
    vulnerability_of(find_package(lockfile, package_name), patched)
}

/// A vulnerability in the locked `package` that is patched in `patched`
fn vulnerability_of(package: &Package, patched: &str) -> Vulnerability {
    let advisory = Advisory::from_str(&format!(
        "```toml\n\
         id = \"RUSTSEC-2026-9999\"\npackage = \"{}\"\n\
         date = \"2026-10-21\"\n\n[versions]\npatched = [\"{patched}\"]\n\
         ```\n\n# Synthetic advisory\n\nFor tests only.\n",
        package.name
    ))
    .unwrap();
    Vulnerability::new(&advisory, package)
}

fn find_package<'a>(lockfile: &'a Lockfile, name: &str) -> &'a Package {
    lockfile
        .packages
        .iter()
        .find(|pkg| pkg.name.as_str() == name)
        .unwrap()
}

/// The locked package called `name` at exactly `version`
fn find_package_version<'a>(lockfile: &'a Lockfile, name: &str, version: &str) -> &'a Package {
    lockfile
        .packages
        .iter()
        .find(|pkg| pkg.name.as_str() == name && pkg.version.to_string() == version)
        .unwrap()
}

/// The program and arguments of `command`, as strings
fn command_line(command: &Command) -> Vec<String> {
    std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect()
}

/// Index whose every lookup fails, as a broken network or disk index would
struct UnavailableIndex;

impl ReleaseIndex for UnavailableIndex {
    fn releases(&mut self, name: &Name) -> Result<Vec<Release>, Error> {
        Err(Error::new(
            ErrorKind::Io,
            format!("index unavailable for {name}"),
        ))
    }
}

/// Plan a fix for the vulnerable `package_name` in `lockfile`
fn plan(
    lockfile: Lockfile,
    package_name: &str,
    patched: &str,
    index: &mut FakeIndex,
) -> DependentFix {
    let vulnerability = vulnerability(&lockfile, package_name, patched);
    let fixer = Fixer::new(lockfile, None, None);
    fixer
        .plan_fix_via_dependents(&vulnerability, index)
        .unwrap()
}

/// Assert that `fix` updates `name` to exactly `version`
fn assert_update(fix: &DependentFix, name: &str, version: &str) {
    match fix {
        DependentFix::Update {
            package,
            version: to,
        } => {
            assert_eq!(package.name.as_str(), name);
            assert_eq!(to.to_string(), version);
        }
        other => panic!("expected an update of {name} to {version}, got {other:?}"),
    }
}

/// `app -> libduckdb-sys 1.0.0 -> zip 6.0.0`
fn one_level_lockfile() -> Lockfile {
    lockfile(&[
        ("app", "0.1.0", &["libduckdb-sys"]),
        ("libduckdb-sys", "1.0.0", &["zip"]),
        ("zip", "6.0.0", &[]),
    ])
}

/// `app -> duckdb 1.0.0 -> libduckdb-sys 1.0.0 -> zip 6.0.0`
fn two_level_lockfile() -> Lockfile {
    lockfile(&[
        ("app", "0.1.0", &["duckdb"]),
        ("duckdb", "1.0.0", &["libduckdb-sys"]),
        ("libduckdb-sys", "1.0.0", &["zip"]),
        ("zip", "6.0.0", &[]),
    ])
}

fn zip_releases(index: FakeIndex) -> FakeIndex {
    index
        .publish("zip", "6.0.0", &[])
        .publish("zip", "9.0.5", &[])
}

#[test]
fn updates_vulnerable_crate_itself_when_its_dependents_allow_it() {
    let mut index = FakeIndex::default()
        .publish("libduckdb-sys", "1.0.0", &[("zip", ">=6, <10")])
        .publish("zip", "6.0.0", &[])
        .publish("zip", "9.0.5", &[]);
    let fix = plan(one_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "zip", "9.0.5");
}

#[test]
fn updates_direct_dependent_whose_new_release_requires_patched_version() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^9")]);
    let fix = plan(one_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "libduckdb-sys", "1.1.0");
}

#[test]
fn walks_up_to_grandparent_when_dependent_is_pinned_exactly() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^9")])
        .publish("duckdb", "1.0.0", &[("libduckdb-sys", "=1.0.0")])
        .publish("duckdb", "1.1.0", &[("libduckdb-sys", "=1.1.0")]);
    let fix = plan(two_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "duckdb", "1.1.0");
}

#[test]
fn reports_no_released_fix_when_no_dependent_has_moved_on() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.0.1", &[("zip", "^6")]);
    let fix = plan(one_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_eq!(fix, DependentFix::NoReleasedFix);
}

#[test]
fn picks_lowest_qualifying_release_to_minimize_lockfile_churn() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.2.0", &[("zip", "^9")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^9")]);
    let fix = plan(one_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "libduckdb-sys", "1.1.0");
}

#[test]
fn skips_yanked_semver_incompatible_and_prerelease_dependent_releases() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^9")])
        .yank_latest("libduckdb-sys")
        .publish("libduckdb-sys", "1.2.0-rc.1", &[("zip", "^9")])
        .publish("libduckdb-sys", "2.0.0", &[("zip", "^9")]);
    let fix = plan(one_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_eq!(fix, DependentFix::NoReleasedFix);
}

#[test]
fn skips_dependent_release_that_still_admits_the_locked_vulnerable_version() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", ">=6, <10")])
        .publish("libduckdb-sys", "1.2.0", &[("zip", "^9")]);
    let fix = plan(one_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "libduckdb-sys", "1.2.0");
}

#[test]
fn reports_multiple_blocking_dependents_instead_of_guessing() {
    let lockfile = lockfile(&[
        ("app", "0.1.0", &["libduckdb-sys", "other"]),
        ("libduckdb-sys", "1.0.0", &["zip"]),
        ("other", "1.0.0", &["zip"]),
        ("zip", "6.0.0", &[]),
    ]);
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("other", "1.0.0", &[("zip", "^6")]);
    let fix = plan(lockfile, "zip", ">= 9.0.0", &mut index);
    match fix {
        DependentFix::MultipleBlockingDependents(blockers) => {
            let names: Vec<&str> = blockers.iter().map(|p| p.name.as_str()).collect();
            assert_eq!(names, ["libduckdb-sys", "other"]);
        }
        other => panic!("expected MultipleBlockingDependents, got {other:?}"),
    }
}

#[test]
fn fails_when_locked_dependent_is_missing_from_the_index() {
    let mut index = zip_releases(FakeIndex::default());
    let lockfile = one_level_lockfile();
    let vulnerability = vulnerability(&lockfile, "zip", ">= 9.0.0");
    let fixer = Fixer::new(lockfile, None, None);
    assert!(
        fixer
            .plan_fix_via_dependents(&vulnerability, &mut index)
            .is_err()
    );
}

#[test]
fn precise_update_command_targets_the_planned_package_and_version() {
    let lockfile = one_level_lockfile();
    let package = find_package(&lockfile, "libduckdb-sys").clone();
    let fixer = Fixer::new(lockfile, None, None);
    let fix = DependentFix::Update {
        package: Box::new(package),
        version: "1.1.0".parse().unwrap(),
    };
    let command = fixer.get_fix_via_dependents_command(&fix, true).unwrap();
    let args: Vec<&OsStr> = command.get_args().collect();
    let expected_pkgid = format!("{REGISTRY}#libduckdb-sys@1.0.0");
    assert_eq!(
        args,
        ["update", "--dry-run", &expected_pkgid, "--precise", "1.1.0"]
    );
}

#[test]
fn no_command_when_there_is_nothing_to_update() {
    let fixer = Fixer::new(one_level_lockfile(), None, None);
    assert!(
        fixer
            .get_fix_via_dependents_command(&DependentFix::NoReleasedFix, false)
            .is_none()
    );
}

#[test]
fn reports_no_released_fix_when_no_patched_release_is_published() {
    let lockfile = lockfile(&[
        ("app", "0.1.0", &["libduckdb-sys", "other"]),
        ("libduckdb-sys", "1.0.0", &["zip"]),
        ("other", "1.0.0", &["zip"]),
        ("zip", "6.0.0", &[]),
    ]);
    let mut index = FakeIndex::default()
        .publish("zip", "6.0.0", &[])
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("other", "1.0.0", &[("zip", "^6")]);
    let fix = plan(lockfile, "zip", ">= 9.0.0", &mut index);
    assert_eq!(fix, DependentFix::NoReleasedFix);
}

#[test]
fn never_plans_a_downgrade_to_an_unaffected_older_release() {
    let mut index = FakeIndex::default()
        .publish("zip", "4.0.0", &[])
        .publish("zip", "6.0.0", &[])
        .publish("zip", "9.0.5", &[])
        .publish("libduckdb-sys", "1.0.0", &[("zip", ">=4, <10")]);
    let lockfile = one_level_lockfile();
    let mut vulnerability = vulnerability(&lockfile, "zip", ">= 9.0.0");
    vulnerability.versions =
        serde_json::from_str(r#"{"patched": [">= 9.0.0"], "unaffected": ["< 5.0.0"]}"#).unwrap();
    let fixer = Fixer::new(lockfile, None, None);
    let fix = fixer
        .plan_fix_via_dependents(&vulnerability, &mut index)
        .unwrap();
    assert_update(&fix, "zip", "9.0.5");
}

#[test]
fn reports_manifest_change_when_a_workspace_member_blocks_the_fix() {
    let lockfile = lockfile(&[("app", "0.1.0", &["zip"]), ("zip", "6.0.0", &[])]);
    let mut index = zip_releases(FakeIndex::default());
    let fix = plan(lockfile, "zip", ">= 9.0.0", &mut index);
    match fix {
        DependentFix::ManifestChangeRequired(member) => assert_eq!(member.name.as_str(), "app"),
        other => panic!("expected ManifestChangeRequired, got {other:?}"),
    }
}

/// `app -> {libduckdb-sys 1.0.0, other 1.0.0} -> zip 6.0.0`
fn two_dependents_lockfile() -> Lockfile {
    lockfile(&[
        ("app", "0.1.0", &["libduckdb-sys", "other"]),
        ("libduckdb-sys", "1.0.0", &["zip"]),
        ("other", "1.0.0", &["zip"]),
        ("zip", "6.0.0", &[]),
    ])
}

/// Two copies of zip: 6.0.0 under libduckdb-sys and 2.0.0 under `old`
fn two_zip_versions_lockfile() -> Lockfile {
    lockfile(&[
        ("app", "0.1.0", &["libduckdb-sys", "old"]),
        ("libduckdb-sys", "1.0.0", &["zip 6.0.0"]),
        ("old", "1.0.0", &["zip 2.0.0"]),
        ("zip", "6.0.0", &[]),
        ("zip", "2.0.0", &[]),
        ("zip", "9.0.5", &[]),
    ])
}

#[test]
fn plain_fix_command_is_unchanged_by_the_shared_command_builder() {
    let lockfile = two_zip_versions_lockfile();
    let vulnerability = vulnerability(&lockfile, "zip", ">= 9.0.0");
    let fixer = Fixer::new(
        lockfile,
        Some(PathBuf::from("ws/Cargo.toml")),
        Some(PathBuf::from("/opt/cargo")),
    );
    let command = fixer.get_fix_command(&vulnerability, true);
    assert_eq!(
        command_line(&command),
        [
            "/opt/cargo",
            "update",
            "--manifest-path",
            "ws/Cargo.toml",
            "--dry-run",
            "--",
            &format!("{REGISTRY}#zip@6.0.0"),
            &format!("{REGISTRY}#zip@2.0.0"),
        ]
    );
}

#[test]
fn plain_fix_command_defaults_to_cargo_on_path_without_dry_run() {
    let lockfile = one_level_lockfile();
    let vulnerability = vulnerability(&lockfile, "zip", ">= 9.0.0");
    let fixer = Fixer::new(lockfile, None, None);
    let command = fixer.get_fix_command(&vulnerability, false);
    assert_eq!(
        command_line(&command),
        ["cargo", "update", "--", &format!("{REGISTRY}#zip@6.0.0")]
    );
}

#[test]
fn precise_update_command_honors_manifest_path_and_cargo_path() {
    let lockfile = one_level_lockfile();
    let package = find_package(&lockfile, "libduckdb-sys").clone();
    let fixer = Fixer::new(
        lockfile,
        Some(PathBuf::from("ws/Cargo.toml")),
        Some(PathBuf::from("/opt/cargo")),
    );
    let fix = DependentFix::Update {
        package: Box::new(package),
        version: "1.1.0".parse().unwrap(),
    };
    let command = fixer.get_fix_via_dependents_command(&fix, false).unwrap();
    assert_eq!(
        command_line(&command),
        [
            "/opt/cargo",
            "update",
            "--manifest-path",
            "ws/Cargo.toml",
            &format!("{REGISTRY}#libduckdb-sys@1.0.0"),
            "--precise",
            "1.1.0",
        ]
    );
}

#[test]
fn no_command_for_any_outcome_other_than_update() {
    let lockfile = one_level_lockfile();
    let app = find_package(&lockfile, "app").clone();
    let fixer = Fixer::new(lockfile, None, None);
    let outcomes = [
        DependentFix::ManifestChangeRequired(Box::new(app.clone())),
        DependentFix::MultipleBlockingDependents(vec![app]),
    ];
    for fix in outcomes {
        assert!(fixer.get_fix_via_dependents_command(&fix, false).is_none());
    }
}

#[test]
fn skips_yanked_and_prerelease_patched_releases_of_the_vulnerable_crate() {
    let mut index = FakeIndex::default()
        .publish("libduckdb-sys", "1.0.0", &[("zip", ">=6, <10")])
        .publish("zip", "6.0.0", &[])
        .publish("zip", "9.0.5", &[])
        .yank_latest("zip")
        .publish("zip", "9.0.6-rc.1", &[])
        .publish("zip", "9.0.6", &[]);
    let fix = plan(one_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "zip", "9.0.6");
}

#[test]
fn workspace_member_admits_semver_compatible_patch_under_default_caret() {
    let lockfile = lockfile(&[("app", "0.1.0", &["zip"]), ("zip", "6.0.0", &[])]);
    let mut index = FakeIndex::default()
        .publish("zip", "6.0.0", &[])
        .publish("zip", "6.0.3", &[]);
    let fix = plan(lockfile, "zip", ">= 6.0.3", &mut index);
    assert_update(&fix, "zip", "6.0.3");
}

#[test]
fn lowest_candidate_respects_every_dependent_that_admits_some() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("zip", "10.0.0", &[])
        .publish("libduckdb-sys", "1.0.0", &[("zip", ">=6")])
        .publish("other", "1.0.0", &[("zip", ">=6, <7"), ("zip", "^10")]);
    let fix = plan(two_dependents_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "zip", "10.0.0");
}

#[test]
fn moved_dependent_must_require_a_version_the_other_dependents_admit() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("zip", "10.0.0", &[])
        .publish("other", "1.0.0", &[("zip", ">=6, <10")])
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^10")])
        .publish("libduckdb-sys", "1.2.0", &[("zip", "^9")]);
    let fix = plan(two_dependents_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "libduckdb-sys", "1.2.0");
}

#[test]
fn ignores_dependents_of_other_locked_versions_of_the_vulnerable_crate() {
    let lockfile = two_zip_versions_lockfile();
    let vulnerability =
        vulnerability_of(find_package_version(&lockfile, "zip", "6.0.0"), ">= 9.0.0");
    let mut index = zip_releases(FakeIndex::default())
        .publish("zip", "2.0.0", &[])
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^9")])
        .publish("old", "1.0.0", &[("zip", "^2")]);
    let fixer = Fixer::new(lockfile, None, None);
    let fix = fixer
        .plan_fix_via_dependents(&vulnerability, &mut index)
        .unwrap();
    assert_update(&fix, "libduckdb-sys", "1.1.0");
}

#[test]
fn walks_up_past_a_workspace_member_that_already_admits_the_move() {
    let lockfile = lockfile(&[
        ("app", "0.1.0", &["duckdb", "libduckdb-sys"]),
        ("duckdb", "1.0.0", &["libduckdb-sys"]),
        ("libduckdb-sys", "1.0.0", &["zip"]),
        ("zip", "6.0.0", &[]),
    ]);
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^9")])
        .publish("duckdb", "1.0.0", &[("libduckdb-sys", "=1.0.0")])
        .publish("duckdb", "1.1.0", &[("libduckdb-sys", "=1.1.0")]);
    let fix = plan(lockfile, "zip", ">= 9.0.0", &mut index);
    assert_update(&fix, "duckdb", "1.1.0");
}

#[test]
fn reports_no_released_fix_when_grandparent_has_not_moved_on() {
    let mut index = zip_releases(FakeIndex::default())
        .publish("libduckdb-sys", "1.0.0", &[("zip", "^6")])
        .publish("libduckdb-sys", "1.1.0", &[("zip", "^9")])
        .publish("duckdb", "1.0.0", &[("libduckdb-sys", "=1.0.0")])
        .publish("duckdb", "1.0.1", &[("libduckdb-sys", "=1.0.0")]);
    let fix = plan(two_level_lockfile(), "zip", ">= 9.0.0", &mut index);
    assert_eq!(fix, DependentFix::NoReleasedFix);
}

#[test]
fn propagates_index_errors() {
    let lockfile = one_level_lockfile();
    let vulnerability = vulnerability(&lockfile, "zip", ">= 9.0.0");
    let fixer = Fixer::new(lockfile, None, None);
    let error = fixer
        .plan_fix_via_dependents(&vulnerability, &mut UnavailableIndex)
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Io);
}
