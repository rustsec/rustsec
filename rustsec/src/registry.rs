//! Support for interacting with the local crates.io registry index

use crate::{
    error::{Error, ErrorKind},
    package::{self, Package},
};

// Re-export the cached index implementation
pub use crate::cached_index::CachedIndex;

/// Crates.io registry index (local copy)
pub struct Index(crates_index::Index);

impl Index {
    /// Open the local crates.io index, updating it.
    pub fn fetch() -> Result<Self, Error> {
        let mut index = crates_index::Index::new_cargo_default()?;
        index.update()?;

        Ok(Index(index))
    }

    /// Open the local crates.io index
    pub fn open() -> Result<Self, Error> {
        let index = crates_index::Index::new_cargo_default()?;

        Ok(Index(index))
    }

    /// Find an entry for a particular package in the index
    pub fn find(
        &self,
        package: &package::Name,
        version: &package::Version,
    ) -> Result<IndexPackage, Error> {
        let crate_releases = self.0.crate_(package.as_str()).ok_or_else(|| {
            format_err!(
                ErrorKind::NotFound,
                "no results for: {} {}",
                &package,
                &version
            )
        })?;

        let crate_release = crate_releases
            .versions()
            .iter()
            .find(|crate_version| crate_version.version() == version.to_string())
            .ok_or_else(|| {
                format_err!(
                    ErrorKind::NotFound,
                    "no results for: {} {}",
                    &package,
                    &version
                )
            })?;

        Ok(IndexPackage::from(crate_release))
    }

    /// Is the given package yanked?
    pub fn is_yanked(&self, package: &Package) -> Result<bool, Error> {
        // TODO(tarcieri): check source matches what we expect
        Ok(self.find(&package.name, &package.version)?.is_yanked)
    }

    /// Iterate over the provided packages, returning a vector of the
    /// packages which have been yanked.
    pub fn find_yanked<'a, I>(&self, packages: I) -> Result<Vec<&'a Package>, Error>
    where
        I: IntoIterator<Item = &'a Package>,
    {
        let mut yanked = Vec::new();

        for package in packages {
            if self.is_yanked(package)? {
                yanked.push(package);
            }
        }

        Ok(yanked)
    }
}

/// Release of the package in the crates.io registry
pub struct IndexPackage {
    /// Name of this package
    pub package: package::Name,

    /// Version of this package
    pub version: package::Version,

    /// Is this package yanked?
    pub is_yanked: bool,
}

impl From<&crates_index::Version> for IndexPackage {
    fn from(crate_release: &crates_index::Version) -> IndexPackage {
        IndexPackage {
            package: crate_release.name().parse().unwrap(),
            version: crate_release.version().parse().unwrap(),
            is_yanked: crate_release.is_yanked(),
        }
    }
}
