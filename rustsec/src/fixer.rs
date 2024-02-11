//! Automatically attempt to fix vulnerable dependencies

use crate::{
    error::{Error, ErrorKind},
    vulnerability::Vulnerability,
};
use cargo_lock::Lockfile;
use std::path::{Path, PathBuf};

/// Auto-fixer for vulnerable dependencies
#[cfg_attr(docsrs, doc(cfg(feature = "fix")))]
pub struct Fixer<'a> {
    manifest_path: PathBuf,
    lockfile: &'a Lockfile,
}

impl <'a> Fixer<'a> {
    /// Create a new [`Fixer`] for the given `Cargo.toml` file
    pub fn new(cargo_toml: &Path, cargo_lock: &'a Lockfile) -> Self {
        Self {
            manifest_path: cargo_toml.to_owned(),
            lockfile: cargo_lock,
        }
    }

    /// Attempt to fix the given vulnerability
    pub fn fix(
        &self,
        vulnerability: &Vulnerability,
        dry_run: bool,
    ) -> Result<(), Error> {
        // let version_req = match vulnerability.versions.patched().get(0) {
        //     Some(req) => req,
        //     None => fail!(ErrorKind::Version, "no fixed version available"),
        // };

        // let dependency = cargo_edit::Dependency::new(vulnerability.package.name.as_str())
        //     .set_version(&version_req.to_string());

        // self.manifest.upgrade(&dependency, dry_run, false)?;

        Ok(())
    }
}

/// Returns a Cargo unique identifier for a package.
/// See `cargo help pkgid` for more info.
///
/// We need to pass these to `cargo update` because otherwise
/// the package specification will be ambiguous, and it will refuse to do anything.
fn pkgid(pkg: &cargo_lock::Package) -> String {
    match pkg.source.as_ref() {
        Some(source) => format!("{}#{}@{}", source, pkg.name, pkg.version),
        None => format!("{}@{}", pkg.name, pkg.version),
    }
}
