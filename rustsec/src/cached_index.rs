//! An efficient way to check whether a given package has been yanked,
//! and to look up the release history the fixer plans with
use std::{
    collections::{BTreeSet, HashMap},
    time::Duration,
};

use crate::{
    error::{Error, ErrorKind},
    fixer::{Release, ReleaseIndex},
    package::{self, Package},
};

use tame_index::external::reqwest::ClientBuilder;
use tame_index::utils::flock::{FileLock, LockOptions};

enum Index {
    SparseCached(tame_index::index::SparseIndex),
    SparseRemote(tame_index::index::AsyncRemoteSparseIndex),
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
/// It is also the crates.io-backed [`ReleaseIndex`] for
/// [`Fixer::plan_fix_via_dependents`](crate::Fixer::plan_fix_via_dependents).
///
/// Operations on crates.io index are rather slow.
/// Instead of peforming an index lookup for every version of every crate,
/// this implementation looks up each crate only once and caches the result in memory.
///
/// Please note that this struct will hold a global Cargo package lock while it exists.
/// Cargo operations that download crates (e.g. `cargo update` or even `cargo build`)
/// will not be possible while this lock is held, so drop it before running
/// the commands a [`Fixer`](crate::Fixer) returns.
pub struct CachedIndex {
    index: Index,
    /// Index entry of each crate looked up so far, or `None` for a crate missing from the index.
    /// Versions stay unparsed strings because crates.io contains invalid semver:
    /// <https://github.com/rustsec/rustsec/issues/759>
    cache: HashMap<package::Name, Result<Option<tame_index::IndexKrate>, Error>>,
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
        Self::fetch_inner(lock_timeout, user_agent).map_err(Error::from_tame)
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
        .map_err(Error::from_tame)
    }

    fn fetch_inner(lock_timeout: Duration, user_agent: String) -> Result<Self, tame_index::Error> {
        let si = tame_index::index::SparseIndex::new(tame_index::IndexLocation::new(
            tame_index::IndexUrl::crates_io(None, None, None)?,
        ))?;

        let lock = acquire_cargo_package_lock(lock_timeout)?;

        // note: this would need to change if rustsec ever adds the capability
        // to query other indices that _might_ not support HTTP/2, but
        // hopefully that would never need to happen
        let client = ClientBuilder::default()
            .user_agent(user_agent)
            .build()
            .map_err(tame_index::Error::from)?;

        Ok(Self {
            index: Index::SparseRemote(tame_index::index::AsyncRemoteSparseIndex::new(si, client)),
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
    fn populate_cache(&mut self, mut packages: BTreeSet<&package::Name>) -> Result<(), Error> {
        // only look up info on packages that aren't yet cached
        packages.retain(|pkg| !self.cache.contains_key(pkg));
        match &self.index {
            Index::SparseCached(_) => {
                for pkg in packages {
                    self.insert(pkg.to_owned(), self.index.krate(pkg, &self.lock));
                }
            }
            Index::SparseRemote(rsi) => {
                // Ensure we have a runtime
                let rt = tame_index::external::tokio::runtime::Runtime::new().map_err(|err| {
                    Error::with_source(
                        ErrorKind::Registry,
                        "unable to start a tokio runtime".to_owned(),
                        err,
                    )
                })?;
                let _rt = rt.enter();

                /// This is the timeout per individual crate. If a crate fails to be
                /// requested for a retriable reason then it will be retried until
                /// this time limit is reached
                const REQUEST_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

                let results = rsi
                    .krates_blocking(
                        packages
                            .into_iter()
                            .map(|p| p.as_str().to_owned())
                            .collect(),
                        true,
                        REQUEST_TIMEOUT,
                        &self.lock,
                    )
                    .map_err(|err| {
                        Error::with_source(
                            ErrorKind::Registry,
                            "unable to acquire tokio runtime".to_owned(),
                            err,
                        )
                    })?;

                for (name, res) in results {
                    self.insert(
                        name.parse().expect("this was a package name before"),
                        res.map_err(Error::from_tame),
                    );
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
        self.cache.insert(package, krate_res);
    }

    /// The cached index entry for `name`; looking it up must already have been attempted
    fn cached_krate(&self, name: &package::Name) -> Result<&tame_index::IndexKrate, Error> {
        match self.cache.get(name) {
            Some(Ok(Some(krate))) => Ok(krate),
            Some(Ok(None)) => Err(Error::new(
                ErrorKind::NotFound,
                format!("No such crate in crates.io index: {name}"),
            )),
            Some(Err(err)) => Err(err.clone()),
            None => Err(Error::new(
                ErrorKind::Registry,
                format!("crates.io index lookup was never attempted for {name}"),
            )),
        }
    }

    /// Is the given package yanked?
    fn is_yanked(&mut self, package: &Package) -> Result<bool, Error> {
        if !self.cache.contains_key(&package.name) {
            self.insert(
                package.name.to_owned(),
                self.index.krate(&package.name, &self.lock),
            );
        }

        let version = package.version.to_string();
        self.cached_krate(&package.name)?
            .versions
            .iter()
            .find(|index_version| index_version.version == version)
            .map(|index_version| index_version.is_yanked())
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "No such version in crates.io index: {} {}",
                        package.name, package.version
                    ),
                )
            })
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
        let package_names: BTreeSet<&package::Name> =
            dedup_packages.iter().map(|p| &p.name).collect();
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

/// Each crate is downloaded once, on first request. The fixer only visits the
/// vulnerable crate and the chain of its dependents, so these lookups are not batched.
impl ReleaseIndex for CachedIndex {
    fn releases(&mut self, name: &package::Name) -> Result<Vec<Release>, Error> {
        self.populate_cache(BTreeSet::from([name]))?;
        Ok(releases_from_index_krate(self.cached_krate(name)?))
    }
}

/// Releases recorded in a crates.io index entry, in the form the fixer plans with
fn releases_from_index_krate(krate: &tame_index::IndexKrate) -> Vec<Release> {
    krate
        .versions
        .iter()
        .filter_map(release_from_index_version)
        .collect()
}

/// `None` for versions that are not valid semver, which crates.io still contains:
/// <https://github.com/rustsec/rustsec/issues/759>
fn release_from_index_version(version: &tame_index::IndexVersion) -> Option<Release> {
    Some(Release {
        version: version.version.parse().ok()?,
        yanked: version.is_yanked(),
        dependencies: version
            .deps
            .iter()
            .filter(|dep| dep.kind() != tame_index::krate::DependencyKind::Dev)
            .filter_map(requirement_on_real_crate)
            .collect(),
    })
}

/// `(real crate name, requirement)` of `dep`, or `None` if either does not parse.
///
/// Dropping an unparseable requirement is conservative: the fixer then treats
/// the release as not admitting the dependency, so it can miss a fix but never
/// plans a wrong one.
fn requirement_on_real_crate(
    dep: &tame_index::IndexDependency,
) -> Option<(package::Name, semver::VersionReq)> {
    Some((dep.crate_name().parse().ok()?, dep.req.parse().ok()?))
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

#[cfg(test)]
mod tests {
    use super::*;

    /// One line of a crates.io index file for the crate `demo`
    fn index_line(version: &str, yanked: bool, dependencies: &[String]) -> String {
        format!(
            r#"{{"name":"demo","vers":"{version}","deps":[{}],"cksum":"{}","features":{{}},"yanked":{yanked}}}"#,
            dependencies.join(","),
            "0".repeat(64)
        )
    }

    /// One dependency of an index line; `kind` is `normal`, `build` or `dev`
    fn index_dependency(name: &str, req: &str, kind: &str, extra: &str) -> String {
        format!(
            r#"{{"name":"{name}","req":"{req}","features":[],"optional":false,"default_features":true,"kind":"{kind}"{extra}}}"#
        )
    }

    fn releases_from_lines(lines: &[String]) -> Vec<Release> {
        let krate = tame_index::IndexKrate::from_slice(lines.join("\n").as_bytes()).unwrap();
        releases_from_index_krate(&krate)
    }

    /// `(name, requirement)` pairs of `release`, as strings
    fn dependency_pairs(release: &Release) -> Vec<(String, String)> {
        release
            .dependencies
            .iter()
            .map(|(name, req)| (name.to_string(), req.to_string()))
            .collect()
    }

    fn pair(name: &str, req: &str) -> (String, String) {
        (name.to_owned(), req.to_owned())
    }

    #[test]
    fn index_versions_become_releases_with_their_yanked_flags() {
        let releases = releases_from_lines(&[
            index_line("1.0.0", false, &[]),
            index_line("1.1.0", true, &[]),
        ]);
        let summary: Vec<(String, bool)> = releases
            .iter()
            .map(|release| (release.version.to_string(), release.yanked))
            .collect();
        assert_eq!(
            summary,
            [("1.0.0".to_owned(), false), ("1.1.0".to_owned(), true)]
        );
    }

    #[test]
    fn keeps_normal_build_and_target_specific_dependencies_but_not_dev() {
        let releases = releases_from_lines(&[index_line(
            "1.0.0",
            false,
            &[
                index_dependency("serde", "^1", "normal", ""),
                index_dependency("zip", "^9", "build", ""),
                index_dependency("winapi", "^0.3", "normal", r#","target":"cfg(windows)""#),
                index_dependency("criterion", "^0.5", "dev", ""),
            ],
        )]);
        assert_eq!(
            dependency_pairs(&releases[0]),
            [
                pair("serde", "^1"),
                pair("zip", "^9"),
                pair("winapi", "^0.3")
            ]
        );
    }

    #[test]
    fn records_renamed_dependency_under_its_real_crate_name() {
        let releases = releases_from_lines(&[index_line(
            "1.0.0",
            false,
            &[index_dependency(
                "zip_alias",
                "^9",
                "normal",
                r#","package":"zip""#,
            )],
        )]);
        assert_eq!(dependency_pairs(&releases[0]), [pair("zip", "^9")]);
    }

    #[test]
    fn skips_index_versions_that_are_not_valid_semver() {
        let releases = releases_from_lines(&[
            index_line("0.1", false, &[]),
            index_line("0.2.0", false, &[]),
        ]);
        let versions: Vec<String> = releases.iter().map(|r| r.version.to_string()).collect();
        assert_eq!(versions, ["0.2.0"]);
    }

    #[test]
    fn skips_dependencies_whose_requirement_does_not_parse() {
        let releases = releases_from_lines(&[index_line(
            "1.0.0",
            false,
            &[
                index_dependency("broken", "not a requirement", "normal", ""),
                index_dependency("zip", "^9", "normal", ""),
            ],
        )]);
        assert_eq!(dependency_pairs(&releases[0]), [pair("zip", "^9")]);
    }
}
