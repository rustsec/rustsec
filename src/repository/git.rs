//! Git repository handling for the RustSec advisory DB

pub mod authentication;
pub mod commit;

use self::authentication::with_authentication;
use crate::{
    collection::Collection,
    error::{Error, ErrorKind},
    repository::{Commit, Repository},
};
use std::{fs, path::PathBuf, vec};

/// Location of the RustSec advisory database for crates.io
pub const DEFAULT_URL: &str = "https://github.com/RustSec/advisory-db.git";

/// Number of days after which the repo will be considered stale
pub const DAYS_UNTIL_STALE: usize = 90;

/// Directory under ~/.cargo where the advisory-db repo will be kept
pub(crate) const ADVISORY_DB_DIRECTORY: &str = "advisory-db";

/// Ref for master in the local repository
const LOCAL_MASTER_REF: &str = "refs/heads/master";

/// Ref for master in the remote repository
const REMOTE_MASTER_REF: &str = "refs/remotes/origin/master";

/// Git repository for a Rust advisory DB
pub struct GitRepository {
    /// Path to the Git repository
    path: PathBuf,

    /// Repository object
    repo: git2::Repository,
}

impl Repository for GitRepository {
    /// Get information about the latest commit to the repo
    fn latest_commit(&self) -> Result<Commit, Error> {
        Commit::from_repo_head(self)
    }

    /// Paths to all advisories located in the database
    fn advisories(&self) -> Result<Vec<PathBuf>, Error> {
        let mut paths = vec![];

        for collection in &[Collection::Crates, Collection::Rust] {
            let collection_path = self.path.join(collection.as_str());

            if let Ok(collection_entry) = fs::read_dir(&collection_path) {
                for dir_entry in collection_entry {
                    for advisory_entry in fs::read_dir(dir_entry?.path())? {
                        paths.push(advisory_entry?.path().to_owned());
                    }
                }
            }
        }

        Ok(paths)
    }
}

impl GitRepository {
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
        Self::fetch(DEFAULT_URL, GitRepository::default_path(), true)
    }

    /// Create a new [`GitRepository`] with the given URL and path
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
                let refspec = LOCAL_MASTER_REF.to_owned() + ":" + REMOTE_MASTER_REF;

                // Fetch remote packfiles and update tips
                let mut remote = repo.remote_anonymous(url)?;
                remote.fetch(&[refspec.as_str()], Some(&mut fetch_opts), None)?;

                // Get the current remote tip (as an updated local reference)
                let remote_master_ref = repo.find_reference(REMOTE_MASTER_REF)?;
                let remote_target = remote_master_ref.target().unwrap();

                // Set the local master ref to match the remote
                let mut local_master_ref = repo.find_reference(LOCAL_MASTER_REF)?;
                local_master_ref.set_target(
                    remote_target,
                    &format!(
                        "rustsec: moving master to {}: {}",
                        REMOTE_MASTER_REF, &remote_target
                    ),
                )?;
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

        // Any commits we fetch should always be signed
        // TODO: verify signatures against GitHub's public key
        if latest_commit.signature.is_none() {
            fail!(
                ErrorKind::Repo,
                "no signature on commit {}: {} ({})",
                latest_commit.commit_id,
                latest_commit.summary,
                latest_commit.author
            );
        }

        // Ensure that the upstream repository hasn't gone stale
        if ensure_fresh {
            latest_commit.ensure_fresh()?;
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
}
