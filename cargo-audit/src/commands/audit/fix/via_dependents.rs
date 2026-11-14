//! `cargo audit fix --via-dependents`: plan updates of dependents that pin a
//! vulnerable crate below its fix

use crate::{auditor::open_crates_io_index, config::AuditConfig, prelude::*};
use rustsec::{
    Fixer, Version, Vulnerability, fixer::DependentFix, package::Package, registry::CachedIndex,
};
use std::process::{Command, exit};

/// `cargo update --precise` commands that move the dependents pinning any of
/// `vulnerabilities`, each paired with the name of the package it moves.
///
/// Several advisories often affect the same package and lead to the same
/// update, so each distinct update appears only once.
pub(super) fn plan_dependent_updates(
    config: &AuditConfig,
    fixer: &Fixer,
    vulnerabilities: &[Vulnerability],
    dry_run: bool,
) -> Vec<(String, Command)> {
    let fixes = plan_with_crates_io_index(config, fixer, vulnerabilities);
    merge_updates_per_package(fixes)
        .iter()
        .filter_map(|fix| {
            let DependentFix::Update { package, .. } = fix else {
                return None;
            };
            let command = fixer.get_fix_via_dependents_command(fix, dry_run)?;
            Some((package.name.to_string(), command))
        })
        .collect()
}

/// Plan a fix for each patchable vulnerability while holding the crates.io
/// index. The index is dropped on return, releasing the Cargo package lock
/// that the planned `cargo update` commands need.
fn plan_with_crates_io_index(
    config: &AuditConfig,
    fixer: &Fixer,
    vulnerabilities: &[Vulnerability],
) -> Vec<DependentFix> {
    // Unlike the yanked check, planning cannot go ahead without the index,
    // so say why opening it failed even when the caller asked for quiet output
    let mut config = config.clone();
    config.output.quiet = false;
    let Some(mut index) = open_crates_io_index(&config) else {
        status_err!("the crates.io index is needed to plan updates of dependents");
        exit(2);
    };
    vulnerabilities
        .iter()
        .filter(|vulnerability| !vulnerability.versions.patched().is_empty())
        .filter_map(|vulnerability| plan_one(fixer, vulnerability, &mut index))
        .collect()
}

/// Plan and report the fix for one vulnerability, or `None` if planning failed
fn plan_one(
    fixer: &Fixer,
    vulnerability: &Vulnerability,
    index: &mut CachedIndex,
) -> Option<DependentFix> {
    match fixer.plan_fix_via_dependents(vulnerability, index) {
        Ok(fix) if moves_the_vulnerable_package_itself(&fix, &vulnerability.package) => None,
        Ok(fix) => {
            report_plan(vulnerability, &fix);
            Some(fix)
        }
        Err(err) => {
            status_warn!(
                "Could not plan an update of dependents for {} in crate {}: {}",
                vulnerability.advisory.id,
                vulnerability.package.name,
                err
            );
            None
        }
    }
}

/// Tell the user what the planner decided for `vulnerability`
fn report_plan(vulnerability: &Vulnerability, fix: &DependentFix) {
    let id = &vulnerability.advisory.id;
    let name = &vulnerability.package.name;
    match fix {
        DependentFix::ManifestChangeRequired(blocker) => {
            status_warn!(
                "{}: {} is held back by {}, whose Cargo.toml requirement must be changed by hand",
                id,
                name,
                blocker.name
            );
        }
        DependentFix::MultipleBlockingDependents(blockers) => {
            status_warn!(
                "{}: {} is held back by several dependents ({}); updating them is not attempted",
                id,
                name,
                package_names(blockers)
            );
        }
        DependentFix::NoReleasedFix => {
            status_warn!(
                "{}: no released version of {} or of its dependents allows a patched {}",
                id,
                name,
                name
            );
        }
        DependentFix::Update { package, version } => {
            status_ok!(
                "Planned",
                "{}: update {} {} -> {} so that {} can be patched",
                id,
                package.name,
                package.version,
                version,
                name
            );
        }
    }
}

fn package_names(packages: &[Package]) -> String {
    let names: Vec<&str> = packages.iter().map(|p| p.name.as_str()).collect();
    names.join(", ")
}

/// One [`DependentFix::Update`] per locked package, at the highest version any
/// of `fixes` planned for it, in order of first appearance
fn merge_updates_per_package(fixes: Vec<DependentFix>) -> Vec<DependentFix> {
    let mut merged: Vec<DependentFix> = Vec::new();
    for fix in fixes {
        let DependentFix::Update { package, version } = fix else {
            continue;
        };
        match merged
            .iter_mut()
            .find_map(|m| planned_version_of(m, &package))
        {
            Some(planned) if version > *planned => *planned = version,
            Some(_) => {}
            None => merged.push(DependentFix::Update { package, version }),
        }
    }
    merged
}

/// The version `fix` moves `package` to, if `fix` is an update of `package`
fn planned_version_of<'a>(fix: &'a mut DependentFix, package: &Package) -> Option<&'a mut Version> {
    match fix {
        DependentFix::Update {
            package: planned,
            version,
        } if **planned == *package => Some(version),
        _ => None,
    }
}

/// Does `fix` move the `vulnerable` package itself? The plain `cargo update`
/// already tries that, so it is not an update of a dependent.
fn moves_the_vulnerable_package_itself(fix: &DependentFix, vulnerable: &Package) -> bool {
    matches!(fix, DependentFix::Update { package, .. } if **package == *vulnerable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustsec::Lockfile;
    use std::str::FromStr;

    /// A crates.io package `name` locked at `version`
    fn package(name: &str, version: &str) -> Package {
        let lockfile = Lockfile::from_str(&format!(
            "version = 4\n[[package]]\nname = \"{name}\"\nversion = \"{version}\"\n\
             source = \"registry+https://github.com/rust-lang/crates.io-index\"\n"
        ))
        .unwrap();
        lockfile.packages[0].clone()
    }

    fn update(package: &Package, version: &str) -> DependentFix {
        DependentFix::Update {
            package: Box::new(package.clone()),
            version: version.parse().unwrap(),
        }
    }

    #[test]
    fn merges_repeated_updates_and_drops_other_outcomes() {
        let duckdb = package("duckdb", "1.0.0");
        let fixes = vec![
            update(&duckdb, "1.1.0"),
            DependentFix::NoReleasedFix,
            update(&duckdb, "1.1.0"),
        ];
        assert_eq!(merge_updates_per_package(fixes), [update(&duckdb, "1.1.0")]);
    }

    #[test]
    fn keeps_the_highest_planned_version_of_each_package() {
        let duckdb = package("duckdb", "1.0.0");
        let other = package("other", "2.0.0");
        let fixes = vec![
            update(&duckdb, "1.1.0"),
            update(&other, "2.1.0"),
            update(&duckdb, "1.3.0"),
            update(&duckdb, "1.2.0"),
        ];
        assert_eq!(
            merge_updates_per_package(fixes),
            [update(&duckdb, "1.3.0"), update(&other, "2.1.0")]
        );
    }

    #[test]
    fn recognizes_an_update_of_the_vulnerable_package_itself() {
        let zip = package("zip", "6.0.0");
        let duckdb = package("duckdb", "1.0.0");
        assert!(moves_the_vulnerable_package_itself(
            &update(&zip, "9.0.5"),
            &zip
        ));
        assert!(!moves_the_vulnerable_package_itself(
            &update(&duckdb, "1.1.0"),
            &zip
        ));
        assert!(!moves_the_vulnerable_package_itself(
            &DependentFix::NoReleasedFix,
            &zip
        ));
    }
}
