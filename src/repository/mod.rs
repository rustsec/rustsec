//! Git repository handling for the RustSec advisory DB

use git2;
use std::{env, fs, path::PathBuf, vec};

use error::{Error, ErrorKind};

mod authentication;
mod commit;
mod file;
mod signature;

pub use self::commit::Commit;
pub(crate) use self::file::RepoFile;
pub use self::signature::Signature;

use self::authentication::with_authentication;

/// Location of the RustSec advisory database for crates.io
pub const ADVISORY_DB_REPO_URL: &str = "https://github.com/RustSec/advisory-db.git";

/// Number of days after which the repo will be considered stale
pub const DAYS_UNTIL_STALE: usize = 90;

/// Directory under ~/.cargo where the advisory-db repo will be kept
const ADVISORY_DB_DIRECTORY: &str = "advisory-db";

/// Directory within a repository where crate advisories are stored
const CRATE_ADVISORY_DIRECTORY: &str = "crates";

/// Ref for master in the local repository
#[cfg(feature = "chrono")]
const LOCAL_MASTER_REF: &str = "refs/heads/master";

/// Ref for master in the remote repository
#[cfg(feature = "chrono")]
const REMOTE_MASTER_REF: &str = "refs/remotes/origin/master";

/// Git repository for a Rust advisory DB
pub struct Repository {
    /// Path to the Git repository
    path: PathBuf,

    /// Repository object
    repo: git2::Repository,
}

impl Repository {
    /// Location of the default `advisory-db` repository for crates.io
    pub fn default_path() -> PathBuf {
        use directories::UserDirs;

        if let Some(path) = env::var_os("CARGO_HOME") {
            PathBuf::from(path).join(ADVISORY_DB_DIRECTORY)
        } else if let Some(user_dirs) = UserDirs::new() {
            PathBuf::from(user_dirs.home_dir())
                .join(".cargo")
                .join(ADVISORY_DB_DIRECTORY)
        } else {
            panic!("Can't locate CARGO_HOME!");
        }
    }

    /// Fetch the default repository
    #[cfg(feature = "chrono")]
    pub fn fetch_default_repo() -> Result<Self, Error> {
        Self::fetch(ADVISORY_DB_REPO_URL, Repository::default_path(), true)
    }

    /// Create a new `Repository` with the given URL and path
    #[cfg(feature = "chrono")]
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

            let mut fetch_opts = git2::FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);

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

        // Ensure the repo is in a clean state
        match repo.state() {
            git2::RepositoryState::Clean => Ok(Repository { path, repo }),
            state => fail!(ErrorKind::Repo, "bad repository state: {:?}", state),
        }
    }

    /// Get information about the latest commit to the repo
    pub fn latest_commit(&self) -> Result<Commit, Error> {
        Commit::from_repo_head(self)
    }

    /// Iterate over all of the crate advisories in this repo
    pub(crate) fn crate_advisories(&self) -> Result<Iter, Error> {
        let mut advisory_files = vec![];

        // Iterate over the individual crates in the `crates/` directory
        for crate_entry in fs::read_dir(self.path.join(CRATE_ADVISORY_DIRECTORY))? {
            for advisory_entry in fs::read_dir(crate_entry?.path())? {
                advisory_files.push(RepoFile::new(advisory_entry?.path())?);
            }
        }

        Ok(Iter(advisory_files.into_iter()))
    }
}

/// Iterator over the advisory database
pub(crate) struct Iter(vec::IntoIter<RepoFile>);

impl Iterator for Iter {
    type Item = RepoFile;

    fn next(&mut self) -> Option<RepoFile> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for Iter {
    fn len(&self) -> usize {
        self.0.len()
    }
}
