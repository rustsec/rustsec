//! An efficient way to check whether a given package has been yanked
use std::collections::HashMap;

use crate::{
    error::{Error, ErrorKind},
    package::{self, Package},
};

/// Provides an efficient way to check if the given package has been yanked.
///
/// Operations on crates.io index are rather slow.
/// Instead of peforming an index lookup for every version of every crate,
/// this implementation looks up each crate only once and caches the result.
/// This usually doesn't result in any dramatic performance wins
/// when auditing a single `Cargo.lock` file because the same crate rarely appears multiple times,
/// but makes a huge difference when auditing many `Cargo.lock`s or many binaries.
pub struct CachedIndex {
    index: crates_index::Index,
    /// The inner hash map is logically HashMap<Version, IsYanked>
    /// but we don't parse semver because crates.io registry contains invalid semver:
    /// <https://github.com/rustsec/rustsec/issues/759>
    // The outer map can later be changed to DashMap or some such for thread safety.
    cache: HashMap<package::Name, HashMap<String, bool>>,
}

impl CachedIndex {
    /// Open the local crates.io index, updating it.
    pub fn fetch() -> Result<Self, Error> {
        let mut index = crates_index::Index::new_cargo_default()?;
        index.update()?;

        Ok(CachedIndex {
            index,
            cache: Default::default(),
        })
    }

    /// Open the local crates.io index
    pub fn open() -> Result<Self, Error> {
        let index = crates_index::Index::new_cargo_default()?;
        Ok(CachedIndex {
            index,
            cache: Default::default(),
        })
    }

    /// Load all version of the given crate from the crates.io index and put them into the cache
    fn populate_cache(&mut self, package: &package::Name) -> Result<(), Error> {
        let crate_releases = self.index.crate_(package.as_str()).ok_or_else(|| {
            format_err!(
                ErrorKind::NotFound,
                "no such crate in the crates.io index: {}",
                &package,
            )
        })?;

        // We already loaded the full crate information, so populate all the versions in the cache
        let versions: HashMap<String, bool> = crate_releases
            .versions()
            .iter()
            .map(|v| (v.version().to_owned(), v.is_yanked()))
            .collect();
        self.cache.insert(package.to_owned(), versions);
        Ok(())
    }

    /// Is the given package yanked?
    pub fn is_yanked(&mut self, package: &Package) -> Result<bool, Error> {
        let crate_is_cached = { self.cache.contains_key(&package.name) };
        if !crate_is_cached {
            self.populate_cache(&package.name)?
        };
        match &self.cache[&package.name].get(&package.version.to_string()) {
            Some(is_yanked) => Ok(**is_yanked),
            None => Err(format_err!(
                ErrorKind::NotFound,
                "No such version in crates.io index: {} {}",
                &package.name,
                &package.version
            )),
        }
    }

    /// Iterate over the provided packages, returning a vector of the
    /// packages which have been yanked.
    pub fn find_yanked<'a, I>(&mut self, packages: I) -> Result<Vec<&'a Package>, Error>
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
