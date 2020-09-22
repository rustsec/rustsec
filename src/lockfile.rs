//! Parser for `Cargo.lock` files

pub(crate) mod encoding;
pub mod version;

pub use self::version::ResolveVersion;

use self::encoding::EncodableLockfile;
use crate::{
    error::{Error, ErrorKind},
    metadata::Metadata,
    package::Package,
    patch::Patch,
};
use std::{fs, path::Path, str::FromStr, string::ToString};

#[cfg(feature = "dependency-tree")]
use crate::dependency::Tree;

/// Parsed Cargo.lock file containing dependencies
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lockfile {
    /// Version of the Lockfile
    pub version: ResolveVersion,

    /// Dependencies enumerated in the lockfile
    pub packages: Vec<Package>,

    /// Legacy "root" dependency for backwards compatibility
    pub root: Option<Package>,

    /// Package metadata
    pub metadata: Metadata,

    /// Patches
    pub patch: Patch,
}

impl Lockfile {
    /// Load lock data from a `Cargo.lock` file
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        match fs::read_to_string(path.as_ref()) {
            Ok(s) => s.parse(),
            Err(e) => fail!(
                ErrorKind::Io,
                "couldn't open {}: {}",
                path.as_ref().display(),
                e
            ),
        }
    }

    /// Get the dependency tree for this `Lockfile`. Returns an error if the
    /// contents of this lockfile aren't well structured.
    ///
    /// The `dependency-tree` Cargo feature must be enabled to use this.
    #[cfg(feature = "dependency-tree")]
    pub fn dependency_tree(&self) -> Result<Tree, Error> {
        Tree::new(self)
    }
}

impl FromStr for Lockfile {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        Ok(toml::from_str(toml_string)?)
    }
}

impl ToString for Lockfile {
    fn to_string(&self) -> String {
        EncodableLockfile::from(self).to_string()
    }
}
