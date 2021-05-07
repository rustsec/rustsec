//! Lockfile versions

use super::encoding::EncodablePackage;
use crate::{
    error::{Error, ErrorKind},
    metadata::Metadata,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
    pub(super) fn detect(
        packages: &[EncodablePackage],
        metadata: &Metadata,
    ) -> Result<Self, Error> {
        // V1: look for [[metadata]] keys beginning with checksum
        let is_v1 = metadata.keys().any(|key| key.is_checksum());

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

impl FromStr for ResolveVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "1" => Ok(ResolveVersion::V1),
            "2" => Ok(ResolveVersion::V2),
            _ => fail!(
                ErrorKind::Parse,
                "invalid Cargo.lock format version: `{}`",
                s
            ),
        }
    }
}
