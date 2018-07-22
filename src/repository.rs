//! Git repository handling for the RustSec advisory DB

#[cfg(feature = "chrono")]
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
#[cfg(feature = "chrono")]
use git2::{AutotagOption, FetchOptions, Oid, ResetType};
use git2::{ObjectType, Repository as GitRepository, RepositoryState};
use std::{
    env,
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
            GitRepository::clone(url, &path)?;
        }

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
    /// ID (i.e. SHA-1 hash) of the latest commit
    pub commit_id: String,

    /// Information about the author of a commit
    pub author: String,

    /// Summary message for the commit
    pub summary: String,

    /// Commit time in number of seconds since the UNIX epoch
    #[cfg(feature = "chrono")]
    pub time: DateTime<Utc>,

    /// Signature on the commit (mandatory)
    // TODO: actually verify signatures
    pub signature: Option<Signature>,

    /// Signed data to verify along with this commit
    raw_signed_bytes: Vec<u8>,
}

impl CommitInfo {
    /// Get information about HEAD
    pub fn from_repo_head(repo: &Repository) -> Result<Self, Error> {
        let head = repo.repo.head()?;

        let oid = head.target().ok_or_else(|| {
            err!(
                ErrorKind::Repo,
                "no ref target for: {}",
                repo.path.display()
            )
        })?;

        let commit_id = oid.to_string();
        let commit_object = repo.repo.find_object(oid, Some(ObjectType::Commit))?;
        let commit = commit_object.as_commit().unwrap();
        let author = commit.author().to_string();

        let summary = commit
            .summary()
            .ok_or_else(|| err!(ErrorKind::Repo, "no commit summary for {}", commit_id))?
            .to_owned();

        let (signature, raw_signed_bytes) = match repo.repo.extract_signature(&oid, None) {
            Ok((s, b)) => (Some(Signature::new(&*s)?), Vec::from(&*b)),
            _ => (None, vec![]),
        };

        #[cfg(feature = "chrono")]
        let time = DateTime::from_utc(
            NaiveDateTime::from_timestamp(commit.time().seconds(), 0),
            Utc,
        );

        Ok(CommitInfo {
            commit_id,
            author,
            summary,
            #[cfg(feature = "chrono")]
            time,
            signature,
            raw_signed_bytes,
        })
    }

    /// Get the raw bytes to be verified when verifying a commit signature
    pub fn raw_signed_bytes(&self) -> &[u8] {
        self.raw_signed_bytes.as_ref()
    }

    /// Reset the repository's state to match this commit
    #[cfg(feature = "chrono")]
    fn reset(&self, repo: &Repository) -> Result<(), Error> {
        let commit_object = repo.repo.find_object(
            Oid::from_str(&self.commit_id).unwrap(),
            Some(ObjectType::Commit),
        )?;

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

/// Signatures on commits to the repository
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature(Vec<u8>);

impl Signature {
    /// Parse a signature from a Git commit
    // TODO: actually verify the signature is well-structured
    pub fn new<T: Into<Vec<u8>>>(into_bytes: T) -> Result<Self, Error> {
        Ok(Signature(into_bytes.into()))
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
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
