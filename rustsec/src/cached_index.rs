//! An efficient way to check whether a given package has been yanked
use std::{
    collections::{BTreeSet, HashMap},
    time::Duration,
};

use crate::{
    error::{Error, ErrorKind},
    package::{self, Package},
};

#[cfg(feature = "gix-reqwest")]
mod reqwest_impl;
#[cfg(feature = "gix-reqwest")]
pub use reqwest_impl::RemoteSparseIndex;

#[cfg(feature = "gix-curl")]
mod curl_impl;
#[cfg(feature = "gix-curl")]
pub use curl_impl::RemoteSparseIndex;

type MultiFetchResult = Vec<(package::Name, Result<Option<tame_index::IndexKrate>, Error>)>;

impl RemoteSparseIndex {
    /// Opens the crates.io index
    pub fn with_user(user_agent: String) -> Result<Self, Error> {
        let index = tame_index::IndexUrl::crates_io(None, None, None)
            .and_then(|iu| tame_index::SparseIndex::new(tame_index::IndexLocation::new(iu)))
            .map_err(Error::from_tame)?;

        Self::new(index, user_agent)
    }

    #[inline]
    fn cached_krate(
        &self,
        name: tame_index::KrateName<'_>,
        lock: &FileLock,
    ) -> Result<Option<tame_index::IndexKrate>, tame_index::Error> {
        self.index.cached_krate(name, lock)
    }
}

use tame_index::utils::flock::{FileLock, LockOptions};

enum Index {
    SparseCached(tame_index::index::SparseIndex),
    SparseRemote(RemoteSparseIndex),
}

impl Index {
    #[inline]
    fn krate(
        &self,
        name: &package::Name,
        lock: &FileLock,
    ) -> Result<Option<tame_index::IndexKrate>, Error> {
        let name = name.as_str().try_into().map_err(Error::from_tame)?;
        let res = match self {
            Self::SparseCached(si) => si.cached_krate(name, lock),
            Self::SparseRemote(rsi) => rsi.cached_krate(name, lock),
        }
        .map_err(Error::from_tame)?;

        Ok(res)
    }
}

/// Provides an efficient way to check if the given package has been yanked.
///
/// Operations on crates.io index are rather slow.
/// Instead of peforming an index lookup for every version of every crate,
/// this implementation looks up each crate only once and caches the result in memory.
///
/// Please note that this struct will hold a global Cargo package lock while it exists.
/// Cargo operations that download crates (e.g. `cargo update` or even `cargo build`)
/// will not be possible while this lock is held.
pub struct CachedIndex {
    index: Index,
    /// The inner hash map is logically HashMap<Version, IsYanked>
    /// but we don't parse semver because crates.io registry contains invalid semver:
    /// <https://github.com/rustsec/rustsec/issues/759>
    cache: HashMap<package::Name, Result<Option<HashMap<String, bool>>, Error>>,
    /// The lock we hold on the Cargo cache directory
    lock: FileLock,
}

impl CachedIndex {
    /// Open the local crates.io index
    ///
    /// If this opens a git index, it will perform a fetch to get the latest index
    /// information.
    ///
    /// If this is a sparse index, it will be downloaded later on demand.
    ///
    /// ## Locking
    ///
    /// This function will wait for up to `lock_timeout` for the filesystem lock on the repository.
    /// It will fail with [`rustsec::Error::LockTimeout`](Error) if the lock is still held
    /// after that time.
    ///
    /// If `lock_timeout` is set to `Duration::from_secs(0)`, it will not wait at all,
    /// and instead return an error immediately if it fails to aquire the lock.
    pub fn fetch_as(lock_timeout: Duration, user_agent: String) -> Result<Self, Error> {
        Self::fetch_inner(lock_timeout, user_agent)
    }

    /// Open the local crates.io index
    ///
    /// Synthesizes a user agent string based on the current version of `rustsec`.
    #[deprecated(since = "0.30.0", note = "Use `CachedIndex::fetch_as()` instead")]
    pub fn fetch(lock_timeout: Duration) -> Result<Self, Error> {
        Self::fetch_inner(
            lock_timeout,
            format!("rustsec/{}", env!("CARGO_PKG_VERSION")),
        )
    }

    fn fetch_inner(lock_timeout: Duration, user_agent: String) -> Result<Self, Error> {
        let si = tame_index::index::SparseIndex::new(tame_index::IndexLocation::new(
            tame_index::IndexUrl::crates_io(None, None, None).map_err(Error::from_tame)?,
        ))
        .map_err(Error::from_tame)?;

        let lock = acquire_cargo_package_lock(lock_timeout).map_err(Error::from_tame)?;
        let index = RemoteSparseIndex::new(si, user_agent)?;

        Ok(Self {
            index: Index::SparseRemote(index),
            cache: Default::default(),
            lock,
        })
    }

    /// Open the local crates.io index
    ///
    /// If this opens a git index, it allows reading of index entries from the repository.
    ///
    /// If this is a sparse index, it only allows reading of index entries that are already cached locally.
    ///
    /// ## Locking
    ///
    /// This function will wait for up to `lock_timeout` for the filesystem lock on the repository.
    /// It will fail with [`rustsec::Error::LockTimeout`](Error) if the lock is still held
    /// after that time.
    ///
    /// If `lock_timeout` is set to `std::time::Duration::from_secs(0)`, it will not wait at all,
    /// and instead return an error immediately if it fails to aquire the lock.
    pub fn open(lock_timeout: Duration) -> Result<Self, Error> {
        Self::open_inner(lock_timeout).map_err(Error::from_tame)
    }

    fn open_inner(lock_timeout: Duration) -> Result<Self, tame_index::Error> {
        let si = tame_index::index::SparseIndex::new(tame_index::IndexLocation::new(
            tame_index::IndexUrl::crates_io(None, None, None)?,
        ))?;

        let lock = acquire_cargo_package_lock(lock_timeout)?;

        let index = Index::SparseCached(si);

        Ok(Self {
            index,
            cache: Default::default(),
            lock,
        })
    }

    /// Populates the cache entries for all of the specified crates.
    fn populate_cache(&mut self, mut packages: BTreeSet<package::Name>) -> Result<(), Error> {
        // only look up info on packages that aren't yet cached
        packages.retain(|pkg| !self.cache.contains_key(pkg));
        match &self.index {
            Index::SparseCached(_) => {
                for pkg in packages {
                    let res = self.index.krate(&pkg, &self.lock);
                    self.insert(pkg, res);
                }
            }
            Index::SparseRemote(rsi) => {
                /// This is the timeout per individual crate. If a crate fails to be
                /// requested for a retriable reason then it will be retried until
                /// this time limit is reached
                const REQUEST_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

                let results = rsi.multi_fetch(packages, REQUEST_TIMEOUT, &self.lock)?;

                for (name, res) in results {
                    self.insert(name, res);
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn insert(
        &mut self,
        package: package::Name,
        krate_res: Result<Option<tame_index::IndexKrate>, Error>,
    ) {
        let krate_res = krate_res.map(|ik| {
            ik.map(|ik| {
                ik.versions
                    .into_iter()
                    .map(|v| (v.version.to_string(), v.is_yanked()))
                    .collect()
            })
        });

        self.cache.insert(package, krate_res);
    }

    /// Is the given package yanked?
    fn is_yanked(&mut self, package: &Package) -> Result<bool, Error> {
        if !self.cache.contains_key(&package.name) {
            self.insert(
                package.name.to_owned(),
                self.index.krate(&package.name, &self.lock),
            );
        }

        match &self.cache[&package.name] {
            Ok(Some(ik)) => match ik.get(&package.version.to_string()) {
                Some(is_yanked) => Ok(*is_yanked),
                None => Err(Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "No such version in crates.io index: {} {}",
                        package.name, package.version
                    ),
                )),
            },
            Ok(None) => Err(Error::new(
                ErrorKind::NotFound,
                format!("No such crate in crates.io index: {}", package.name),
            )),
            Err(err) => Err(err.clone()),
        }
    }

    /// Iterate over the provided packages, returning a vector of the
    /// packages which have been yanked.
    ///
    /// This function should be called with many packages at once rather than one by one;
    /// that way it can download the status of a large number of packages at once from the sparse index
    /// very quickly, orders of magnitude faster than requesting packages one by one.
    pub fn find_yanked<'a, I>(&mut self, packages: I) -> Vec<Result<&'a Package, Error>>
    where
        I: IntoIterator<Item = &'a Package>,
    {
        let mut yanked = Vec::new();

        let dedup_packages: BTreeSet<&Package> = packages.into_iter().collect();
        let package_names: BTreeSet<package::Name> =
            dedup_packages.iter().map(|p| p.name.clone()).collect();
        if let Err(e) = self.populate_cache(package_names) {
            yanked.push(Err(Error::with_source(
                ErrorKind::Registry,
                "Failed to download crates.io index. \
                    Data may be missing or stale when checking for yanked packages."
                    .to_owned(),
                e,
            )));
        }

        for package in dedup_packages {
            match self.is_yanked(package) {
                Ok(false) => {} // not yanked, nothing to report
                Ok(true) => yanked.push(Ok(package)),
                Err(error) => yanked.push(Err(error)),
            }
        }

        yanked
    }
}

// We cannot expose these publicly because that would leak the `tame_index` SemVer into the public API
fn acquire_cargo_package_lock(lock_timeout: Duration) -> Result<FileLock, tame_index::Error> {
    let lock_opts = LockOptions::cargo_package_lock(None)?.exclusive(false);
    acquire_lock(lock_opts, lock_timeout)
}

fn acquire_lock(
    lock_opts: LockOptions<'_>,
    lock_timeout: Duration,
) -> Result<FileLock, tame_index::Error> {
    if lock_timeout == Duration::from_secs(0) {
        lock_opts.try_lock()
    } else {
        lock_opts.lock(|_| Some(lock_timeout))
    }
}
