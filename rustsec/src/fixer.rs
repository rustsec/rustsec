//! Automatically attempt to fix vulnerable dependencies

use crate::{
    error::{Error, ErrorKind},
    vulnerability::Vulnerability,
};
use semver::VersionReq;
use std::path::Path;

/// Auto-fixer for vulnerable dependencies
#[cfg_attr(docsrs, doc(cfg(feature = "fix")))]
pub struct Fixer {
    manifest: cargo_edit::LocalManifest,
}

impl Fixer {
    /// Create a new [`Fixer`] for the given `Cargo.toml` file
    pub fn new(cargo_toml: impl AsRef<Path>) -> Result<Self, Error> {
        let manifest =
            cargo_edit::LocalManifest::try_new(cargo_toml.as_ref().canonicalize()?.as_ref())?;
        Ok(Self { manifest })
    }

    /// Attempt to fix the given vulnerability
    pub fn fix(
        &mut self,
        vulnerability: &Vulnerability,
        dry_run: bool,
    ) -> Result<VersionReq, Error> {
        // TODO(tarcieri): find semver-compatible fix?
        let version_req = match vulnerability.versions.patched().get(0) {
            Some(req) => req,
            None => fail!(ErrorKind::Version, "no fixed version available"),
        };

        let dependency = cargo_edit::Dependency::new(vulnerability.package.name.as_str())
            .set_version(&version_req.to_string());

        self.manifest.upgrade(&dependency, dry_run, false)?;

        // TODO(tarcieri): return new version rather than req?
        Ok(version_req.clone())
    }
}
