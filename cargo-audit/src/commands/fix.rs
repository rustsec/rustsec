//! The `cargo audit fix` subcommand

use std::{
    collections::BTreeMap,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};

use cargo_lock::{Lockfile, Package};
use clap::Parser;
use rustsec::{
    Fixer, Report,
    advisory::{Id, Versions},
};

use crate::{
    auditor::Auditor,
    config::AuditConfig,
    error::Error,
    lockfile,
    terminal::{status_err, status_ok, status_warn},
};

/// The `cargo audit fix` subcommand
#[derive(Clone, Default, Debug, Parser)]
#[command(author, version, about)]
pub struct FixCommand {
    /// Path to `Cargo.lock`
    #[arg(short = 'f', long = "file", help = "Cargo lockfile to inspect")]
    pub(super) file: Option<PathBuf>,

    /// Perform a dry run
    #[arg(long = "dry-run", help = "perform a dry run for the fix")]
    dry_run: bool,
}

impl FixCommand {
    /// Locate `Cargo.lock`
    fn cargo_lock_path(&self) -> Option<&Path> {
        self.file.as_deref()
    }

    fn initial_lockfile(&self) -> io::Result<PathBuf> {
        if let Some(path) = self.cargo_lock_path() {
            return path.canonicalize();
        }
        let path = Path::new("Cargo.lock");
        if path.try_exists()? || !Path::new("Cargo.toml").try_exists()? {
            return path.canonicalize();
        }

        let workspace_lockfile = locate_workspace_lockfile(OsStr::new("cargo"), None)?;
        if !self.dry_run && !workspace_lockfile.try_exists()? {
            lockfile::generate().map_err(io::Error::other)?;
        }
        workspace_lockfile.canonicalize()
    }

    /// Run the `cargo audit fix` subcommand with the given configuration
    pub fn run(self, mut config: AuditConfig) -> Result<ExitCode, Error> {
        let requested_path = self.cargo_lock_path();
        let path = match self.initial_lockfile() {
            Ok(path) => path,
            Err(e) => {
                let path = requested_path.unwrap_or_else(|| Path::new("Cargo.lock"));
                status_err!("Couldn't load {}: {}", path.display(), e);
                return Ok(ExitCode::from(2));
            }
        };

        let report = Auditor::new(&config)?.audit_lockfile(&path);
        let report = match report {
            Ok(report) => report,
            Err(e) => {
                status_err!("{}", e);
                return Ok(ExitCode::from(2));
            }
        };

        let mut advisories = FixTarget::from_report(&report).peekable();
        if advisories.peek().is_none() {
            return Ok(ExitCode::SUCCESS);
        }

        let path_to_cargo = std::env::var_os("CARGO").map(PathBuf::from);
        let dry_run = self.dry_run;
        if dry_run {
            status_warn!("Performing a dry run, the fixes will not be applied");
        }

        let mut has_unpatchable_advisory = false;
        let mut update_failed_to_start = false;
        let mut fixed_by_id = BTreeMap::<_, usize>::new();
        let update_dir = requested_path.and_then(|_| path.parent());
        let mut workspace_lockfile: Option<PathBuf> = None;

        for FixTarget(id, package, versions) in advisories {
            if !dry_run {
                *fixed_by_id.entry(id).or_default() += 1;
            }
            if versions.patched().is_empty() {
                has_unpatchable_advisory = true;
                status_warn!(
                    "No patched versions available for {} in crate {}",
                    id,
                    package.name
                );
            } else {
                // A previous update may have removed an affected version.
                let lockfile = Lockfile::load(workspace_lockfile.as_deref().unwrap_or(&path))?;
                let fixer = Fixer::new(lockfile, None, path_to_cargo.clone());
                let Some(mut command) = fixer.fix_command(package, versions, dry_run) else {
                    continue;
                };
                if workspace_lockfile.is_none() {
                    workspace_lockfile =
                        match checked_lockfile(&path, update_dir, command.get_program()) {
                            Ok(lockfile) => Some(lockfile),
                            Err(e) => {
                                status_err!("{}", e);
                                return Ok(ExitCode::from(2));
                            }
                        };
                }
                if let Some(dir) = update_dir {
                    command.current_dir(dir);
                }
                // When calling `.status()` the stdout and stderr are inherited from the parent,
                // so any status or error messages from `cargo update` will automatically be forwarded
                // to the user of `cargo audit fix`.
                let status = command.status();
                if let Err(e) = status {
                    update_failed_to_start = true;
                    status_warn!(
                        "Failed to run `cargo update` for package {}: {}",
                        package.name,
                        e
                    );
                }
            }
        }

        if update_failed_to_start {
            return Ok(ExitCode::from(2));
        }
        if dry_run {
            Ok(if has_unpatchable_advisory {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        } else {
            status_ok!("Verifying", "which advisories were fixed");
            config.output.quiet = true;
            let mut auditor = Auditor::new(&config)?;

            let report_after_fix =
                auditor.audit_lockfile(workspace_lockfile.as_deref().unwrap_or(&path))?;
            let mut remaining = 0;
            let mut fixable_but_unfixed = Vec::new();
            for FixTarget(id, _, versions) in FixTarget::from_report(&report_after_fix) {
                remaining += 1;
                if let Some(count) = fixed_by_id.get_mut(id) {
                    *count = count.saturating_sub(1);
                }
                if !versions.patched().is_empty() {
                    fixable_but_unfixed.push(id.as_str());
                }
            }
            if !fixable_but_unfixed.is_empty() {
                status_warn!(
                    "The following advisories have patched versions but could not be fixed:\n    {}\n\
                    Check whether your Cargo.toml version requirements allow the patched versions.",
                    fixable_but_unfixed.join(", ")
                );
            }

            let fixed: usize = fixed_by_id.values().sum();
            if fixed != 0 {
                let noun = if fixed == 1 { "advisory" } else { "advisories" };
                if remaining == 0 {
                    status_ok!("Fixed", format!("{fixed} {noun}"));
                } else {
                    status_warn!(format!("Fixed {fixed} {noun} ({remaining} remaining)"));
                }
            }

            Ok(match remaining {
                0 => ExitCode::SUCCESS,
                _ => ExitCode::FAILURE,
            })
        }
    }
}

fn checked_lockfile(lockfile: &Path, dir: Option<&Path>, cargo: &OsStr) -> io::Result<PathBuf> {
    if lockfile.ends_with("Cargo.lock") {
        let workspace_lockfile = locate_workspace_lockfile(cargo, dir)?;
        if workspace_lockfile == lockfile || workspace_lockfile.canonicalize()? == lockfile {
            return Ok(workspace_lockfile);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "`cargo audit fix` requires the workspace's Cargo.lock",
    ))
}

fn locate_workspace_lockfile(cargo: &OsStr, dir: Option<&Path>) -> io::Result<PathBuf> {
    #[derive(serde::Deserialize)]
    struct Project {
        root: PathBuf,
    }

    let mut command = Command::new(cargo);
    if let Some(dir) = dir {
        command.current_dir(dir);
    }
    let output = command
        .args(["locate-project", "--workspace", "--message-format=json"])
        .stderr(Stdio::inherit())
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "`cargo locate-project` exited with {}",
            output.status
        )));
    }
    let project: Project = serde_json::from_slice(&output.stdout).map_err(io::Error::other)?;
    Ok(project.root.with_file_name("Cargo.lock"))
}

struct FixTarget<'a>(&'a Id, &'a Package, &'a Versions);

impl<'a> FixTarget<'a> {
    fn from_report(report: &'a Report) -> impl Iterator<Item = Self> {
        report
            .vulnerabilities
            .list
            .iter()
            .map(|v| Self(&v.advisory.id, &v.package, &v.versions))
            .chain(report.warnings.values().flatten().filter_map(|warning| {
                let id = &warning.advisory.as_ref()?.id;
                let versions = warning.versions.as_ref()?;
                (!versions.patched().is_empty()).then_some(Self(id, &warning.package, versions))
            }))
    }
}
