//! Git repositories

use super::{with_authentication, Commit, DEFAULT_URL};
use crate::{
    error::{Error, ErrorKind},
    fs,
};
use std::path::{Path, PathBuf};

/// Directory under `~/.cargo` where the advisory-db repo will be kept
const ADVISORY_DB_DIRECTORY: &str = "advisory-db";

/// Ref for the `main` branch in the local repository
const LOCAL_REF: &str = "refs/heads/main";

/// Ref for the `main` branch in the remote repository
const REMOTE_REF: &str = "refs/remotes/origin/main";

/// Git repository for a Rust advisory DB.
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
pub struct Repository {
    /// Path to the Git repository
    pub(super) path: PathBuf,

    /// Repository object
    pub(super) repo: git2::Repository,
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

        let git_config = git2::Config::new()?;

        with_authentication(url, &git_config, |f| {
            let mut callbacks = git2::RemoteCallbacks::new();
            callbacks.credentials(f);

            let mut proxy_opts = git2::ProxyOptions::new();
            proxy_opts.auto();

            let mut fetch_opts = git2::FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);
            fetch_opts.proxy_options(proxy_opts);

            if path.exists() {
                let repo = git2::Repository::open(&path)?;
                let refspec = LOCAL_REF.to_owned() + ":" + REMOTE_REF;

                // Fetch remote packfiles and update tips
                let mut remote = repo.remote_anonymous(url)?;
                remote.fetch(&[refspec.as_str()], Some(&mut fetch_opts), None)?;

                // Get the current remote tip (as an updated local reference)
                let remote_main_ref = repo.find_reference(REMOTE_REF)?;
                let remote_target = remote_main_ref.target().unwrap();

                // Set the local main ref to match the remote
                match repo.find_reference(LOCAL_REF) {
                    Ok(mut local_main_ref) => {
                        local_main_ref.set_target(
                            remote_target,
                            &format!(
                                "rustsec: moving `main` to {}: {}",
                                REMOTE_REF, &remote_target
                            ),
                        )?;
                    }
                    Err(e) if e.code() == git2::ErrorCode::NotFound => {
                        // TODO(tarcieri): remove this workaround after repos have migrated
                        let old_ref = repo.find_reference("refs/heads/master")?;
                        git2::Branch::wrap(old_ref).rename("main", true)?;
                        Self::fetch(url, &path, ensure_fresh)?;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                };
            } else {
                git2::build::RepoBuilder::new()
                    .fetch_options(fetch_opts)
                    .clone(url, &path)?;
            }

            Ok(())
        })?;

        let repo = Self::open(path)?;
        let latest_commit = repo.latest_commit()?;
        latest_commit.reset(&repo)?;

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
        let repo = git2::Repository::open(&path)?;

        if repo.state() == git2::RepositoryState::Clean {
            Ok(Self { path, repo })
        } else {
            fail!(ErrorKind::Repo, "bad repository state: {:?}", repo.state())
        }
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
