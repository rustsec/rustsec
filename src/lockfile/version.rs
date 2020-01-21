//! Lockfile versions

use crate::{
    error::{Error, ErrorKind},
    metadata::Metadata,
    package::Package,
};
use serde::{Deserialize, Serialize};

/// Lockfile versions
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum ResolveVersion {
    /// The original `Cargo.lock` format which places checksums in the
    /// `[[metadata]]` table.
    V1,

    /// The new `Cargo.lock` format which is optimized to prevent merge
    /// conflicts. For more information, see:
    ///
    /// <https://github.com/rust-lang/cargo/pull/7070>
    V2,
}

impl ResolveVersion {
    /// Autodetect the version of a lockfile from the packages
    pub fn detect(packages: &[Package], metadata: &Metadata) -> Result<Self, Error> {
        // V1: look for [[metadata]] keys beginning with checksum
        let is_v1 = metadata
            .keys()
            .any(|key| key.as_ref().starts_with("checksum "));

        // V2: look for `checksum` fields in `[package]`
        let is_v2 = packages.iter().any(|package| package.checksum.is_some());

        if is_v1 && is_v2 {
            fail!(ErrorKind::Parse, "malformed lockfile: contains checksums in both [[package]] and [[metadata]] sections");
        }

        if is_v1 {
            Ok(ResolveVersion::V1)
        } else {
            // Default to V2
            Ok(ResolveVersion::V2)
        }
    }
}

/// V2 format is now the default.
///
///See: <https://github.com/rust-lang/cargo/pull/7579>
impl Default for ResolveVersion {
    fn default() -> Self {
        ResolveVersion::V2
    }
}
