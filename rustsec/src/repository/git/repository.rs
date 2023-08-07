//! Git repositories

use super::{with_authentication, Commit, DEFAULT_URL};
use crate::{
    error::{Error, ErrorKind},
    fs,
};
use std::path::{Path, PathBuf};

/// Directory under `~/.cargo` where the advisory-db repo will be kept
const ADVISORY_DB_DIRECTORY: &str = "advisory-db";

/// Refspec used to fetch updates from remote advisory databases
const REF_SPEC: &str = "+HEAD:refs/remotes/origin/HEAD";

/// The direction of the remote
const DIR: gix::remote::Direction = gix::remote::Direction::Fetch;

/// Git repository for a Rust advisory DB.
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
pub struct Repository {
    /// Repository object
    pub(super) repo: gix::Repository,
}

impl Repository {
    /// Location of the default `advisory-db` repository for crates.io
    pub fn default_path() -> PathBuf {
        home::cargo_home()
            .unwrap_or_else(|err| {
                panic!("Error locating Cargo home directory: {}", err);
            })
            .join(ADVISORY_DB_DIRECTORY)
    }

    /// Fetch the default repository
    pub fn fetch_default_repo() -> Result<Self, Error> {
        Self::fetch(DEFAULT_URL, Repository::default_path(), true)
    }

    /// Create a new [`Repository`] with the given URL and path
    pub fn fetch<P: Into<PathBuf>>(
        url: &str,
        into_path: P,
        ensure_fresh: bool,
    ) -> Result<Self, Error> {
        if !url.starts_with("https://") {
            fail!(
                ErrorKind::BadParam,
                "expected {} to start with https://",
                url
            );
        }

        let path = into_path.into();

        if let Some(parent) = path.parent() {
            if !parent.is_dir() {
                fs::create_dir_all(parent)?;
            }
        } else {
            fail!(ErrorKind::BadParam, "invalid directory: {}", path.display())
        }

        // Avoid libgit2 errors in the case the directory exists but is
        // otherwise empty.
        //
        // See: https://github.com/RustSec/cargo-audit/issues/32
        if path.is_dir() && fs::read_dir(&path)?.next().is_none() {
            fs::remove_dir(&path)?;
        }
        let _lock = gix::lock::Marker::acquire_to_hold_resource(
            path.with_extension("rustsec"),
            gix::lock::acquire::Fail::AfterDurationWithBackoff(std::time::Duration::from_secs(
                60 * 10, /* 10 minutes */
            )),
            Some(std::path::PathBuf::from_iter(Some(
                std::path::Component::RootDir,
            ))),
        )
        .map_err(|err| format_err!(ErrorKind::Repo, "unable to acquire repo lock: {}", err))?;

        let open_or_clone_repo = || -> Result<_, Error> {
            let mut mapping = gix::sec::trust::Mapping::default();
            let open_with_complete_config =
                gix::open::Options::default().permissions(gix::open::Permissions {
                    config: gix::open::permissions::Config {
                        // Be sure to get all configuration, some of which is only known by the git binary.
                        // That way we are sure to see all the systems credential helpers
                        git_binary: true,
                        ..Default::default()
                    },
                    ..Default::default()
                });

            mapping.reduced = open_with_complete_config.clone();
            mapping.full = open_with_complete_config.clone();

            // Attempt to open the repository, if it fails for any reason,
            // attempt to perform a fresh clone instead
            let repo = gix::ThreadSafeRepository::discover_opts(
                path,
                gix::discover::upwards::Options::default().apply_environment(),
                mapping,
            )
            .ok()
            .map(|repo| repo.to_thread_local())
            .filter(|repo| {
                repo.find_remote("origin").map_or(false, |remote| {
                    remote
                        .url(DIR)
                        .map_or(false, |remote_url| remote_url.to_bstring() == url)
                })
            })
            .or_else(|| gix::open_opts(path, open_with_complete_config).ok());

            let res = if let Some(repo) = repo {
                (repo, None)
            } else {
                let mut progress = gix::progress::Discard;
                let should_interrupt = &gix::interrupt::IS_INTERRUPTED;

                let (mut prep_checkout, out) = gix::prepare_clone(url, path)
                    .map_err(|err| {
                        format_err!(ErrorKind::Repo, "failed to prepare clone: {}", err)
                    })?
                    .with_remote_name("origin")
                    .map_err(|err| format_err!(ErrorKind::Repo, "invalid remote name: {}", err))?
                    .configure_remote(|remote| Ok(remote.with_refspecs([REF_SPEC], DIR)?))
                    .fetch_then_checkout(&mut progress, should_interrupt)
                    .map_err(|err| format_err!(ErrorKind::Repo, "failed to fetch repo: {}", err))?;

                let repo = prep_checkout
                    .main_worktree(&mut progress, should_interrupt)
                    .map_err(|err| {
                        format_err!(ErrorKind::Repo, "failed to checkout fresh clone: {}", err)
                    })?
                    .0;

                (repo, Some(out))
            };

            Ok(res)
        };

        let (mut repo, fetch_outcome) = open_or_clone_repo()?;

        if let Some(fetch_outcome) = fetch_outcome {
            tame_index::utils::git::write_fetch_head(
                &repo,
                &fetch_outcome,
                &repo.find_remote("origin").unwrap(),
            )?;
        } else {
            // If we didn't open a fresh repo we need to peform a fetch ourselves, and
            // do the work of updating the HEAD to point at the latest remote HEAD, which
            // gix doesn't currently do.
            Self::fetch_and_checkout(&mut repo)?;
        }

        repo.object_cache_size_if_unset(4 * 1024 * 1024);
        let repo = Self { repo };

        let latest_commit = Commit::from_repo_head(&repo)?;

        // Ensure that the upstream repository hasn't gone stale
        if ensure_fresh && !latest_commit.is_fresh() {
            fail!(
                ErrorKind::Repo,
                "repository is stale (last commit: {:?})",
                latest_commit.timestamp
            );
        }

        Ok(repo)
    }

    /// Open a repository at the given path
    pub fn open<P: Into<PathBuf>>(into_path: P) -> Result<Self, Error> {
        let path = into_path.into();
        let repo = gix::open(&path)?;

        // TODO: Figure out how to detect if the worktree has modifications
        // as gix currently doesn't have a status/state summary like git2 has
        Ok(Self { repo })
    }

    /// Get information about the latest commit to the repo
    pub fn latest_commit(&self) -> Result<Commit, Error> {
        Commit::from_repo_head(self)
    }

    /// Path to the local checkout of a git repository
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
