//! Git repository handling for the RustSec advisory DB

#[cfg(feature = "chrono")]
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use git2::{
    AutotagOption, FetchOptions, ObjectType, Oid, Repository as GitRepository, RepositoryState,
    ResetType,
};
use std::{
    env,
    fmt::Write,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    vec,
};

use error::{Error, ErrorKind};

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
    repo: GitRepository,
}

impl Repository {
    /// Location of the default `advisory-db` repository for crates.io
    pub fn default_path() -> PathBuf {
        if let Some(path) = env::var_os("CARGO_HOME") {
            PathBuf::from(path).join(ADVISORY_DB_DIRECTORY)
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
                fail!(ErrorKind::BadParam, "not a directory: {}", parent.display());
            }
        } else {
            fail!(ErrorKind::BadParam, "invalid directory: {}", path.display())
        }

        if path.exists() {
            let repo = GitRepository::open(&path)?;
            let refspec = LOCAL_MASTER_REF.to_owned() + ":" + REMOTE_MASTER_REF;

            let mut fetch_opts = FetchOptions::new();
            fetch_opts.download_tags(AutotagOption::All);

            // Fetch remote packfiles and update tips
            let mut remote = repo.remote_anonymous(url)?;
            remote.fetch(&[refspec.as_str()], Some(&mut fetch_opts), None)?;

            // Get the current remote tip (as an updated local reference)
            let remote_master_ref = repo.find_reference(REMOTE_MASTER_REF)?;
            let remote_target = remote_master_ref.target().unwrap();
            let remote_target_hex = oid_to_hex(remote_target);

            // Set the local master ref to match the remote
            let mut local_master_ref = repo.find_reference(LOCAL_MASTER_REF)?;
            local_master_ref.set_target(
                remote_target,
                &format!(
                    "rustsec: moving master to {}: {}",
                    REMOTE_MASTER_REF, &remote_target_hex
                ),
            )?;
        } else {
            GitRepository::clone(url, &path)?;
        }

        let repo = Self::open(path)?;
        let latest_commit = repo.latest_commit()?;
        latest_commit.reset(&repo)?;

        // Ensure that the upstream repository hasn't gone stale
        if ensure_fresh {
            latest_commit.ensure_fresh()?;
        }

        Ok(repo)
    }

    /// Open a repository at the given path
    pub fn open<P: Into<PathBuf>>(into_path: P) -> Result<Self, Error> {
        let path = into_path.into();
        let repo = GitRepository::open(&path)?;

        // Ensure the repo is in a clean state
        match repo.state() {
            RepositoryState::Clean => Ok(Repository { path, repo }),
            state => fail!(ErrorKind::Repo, "bad repository state: {:?}", state),
        }
    }

    /// Get information about the latest commit to the repo
    pub fn latest_commit(&self) -> Result<CommitInfo, Error> {
        CommitInfo::from_repo_head(self)
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

/// Information about a commit to the Git repository
#[derive(Debug)]
pub struct CommitInfo {
    /// git2 object identifier
    pub oid: Oid,

    /// ID (i.e. SHA-1 hash) of the latest commit
    pub commit_id: String,

    /// Name of the current ref
    pub ref_name: String,

    /// Summary message for the commit
    pub summary: String,

    /// Commit time in number of seconds since the UNIX epoch
    #[cfg(feature = "chrono")]
    pub time: DateTime<Utc>,
}

impl CommitInfo {
    /// Get information about HEAD
    pub fn from_repo_head(repo: &Repository) -> Result<Self, Error> {
        let head = repo.repo.head()?;

        let ref_name = head
            .name()
            .ok_or_else(|| {
                err!(
                    ErrorKind::Repo,
                    "no current ref name for: {}",
                    repo.path.display()
                )
            })?
            .to_owned();

        let oid = head.target().ok_or_else(|| {
            err!(
                ErrorKind::Repo,
                "no ref target for: {}",
                repo.path.display()
            )
        })?;

        let commit_id = oid_to_hex(oid);
        let commit_object = repo.repo.find_object(oid, Some(ObjectType::Commit))?;
        let commit = commit_object.as_commit().unwrap();

        let summary = commit
            .summary()
            .ok_or_else(|| err!(ErrorKind::Repo, "no commit summary for {}", commit_id))?
            .to_owned();

        #[cfg(feature = "chrono")]
        let time = DateTime::from_utc(
            NaiveDateTime::from_timestamp(commit.time().seconds(), 0),
            Utc,
        );

        Ok(CommitInfo {
            oid,
            commit_id,
            ref_name,
            summary,
            #[cfg(feature = "chrono")]
            time,
        })
    }

    /// Reset the repository's state to match this commit
    fn reset(&self, repo: &Repository) -> Result<(), Error> {
        let commit_object = repo.repo.find_object(self.oid, Some(ObjectType::Commit))?;

        // Reset the state of the repository to the latest commit
        repo.repo.reset(&commit_object, ResetType::Hard, None)?;

        Ok(())
    }

    /// Determine if the repository is fresh or stale (i.e. has it recently been committed to)
    #[cfg(feature = "chrono")]
    fn ensure_fresh(&self) -> Result<(), Error> {
        let fresh_after_date = Utc::now()
            .checked_sub_signed(Duration::days(DAYS_UNTIL_STALE as i64))
            .unwrap();

        if self.time > fresh_after_date {
            Ok(())
        } else {
            fail!(
                ErrorKind::Repo,
                "stale repo: not updated for {} days (last commit: {:?})",
                DAYS_UNTIL_STALE,
                self.time
            )
        }
    }
}

/// File stored in the repository
#[derive(Debug)]
pub(crate) struct RepoFile(PathBuf);

impl RepoFile {
    /// Create a RepoFile from a relative repo `Path` and a `File`
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<RepoFile, Error> {
        Ok(RepoFile(path.into()))
    }

    /// Path to this file on disk
    pub fn path(&self) -> &Path {
        self.0.as_ref()
    }

    /// Read the file to a string
    pub fn read_to_string(&self) -> Result<String, Error> {
        let mut file = File::open(&self.0)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        Ok(string)
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

fn oid_to_hex(oid: Oid) -> String {
    let mut hex = String::new();

    for byte in oid.as_bytes().iter() {
        write!(hex, "{:02x}", byte).unwrap();
    }

    hex
}
