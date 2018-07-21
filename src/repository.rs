//! Git repository handling for the RustSec advisory DB

#[cfg(feature = "chrono")]
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use git2::{ObjectType, Repository as GitRepository, RepositoryState};
use std::{
    env,
    fmt::Write,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use error::{Error, ErrorKind};

/// Location of the RustSec advisory database for crates.io
pub const ADVISORY_DB_REPO_URL: &str = "https://github.com/RustSec/advisory-db.git";

/// Number of days after which the repo will be considered stale
pub const DAYS_UNTIL_STALE: usize = 90;

/// Directory under ~/.cargo where the advisory-db repo will be kept
const ADVISORY_DB_DIRECTORY: &str = "advisory-db";

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
        Self::fetch(ADVISORY_DB_REPO_URL, Repository::default_path())
    }

    /// Create a new `Repository` with the given URL and path
    #[cfg(feature = "chrono")]
    pub fn fetch<P: Into<PathBuf>>(url: &str, into_path: P) -> Result<Self, Error> {
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
            repo.remote_anonymous(url)?
                .fetch(&[refspec.as_str()], None, None)?;
        } else {
            GitRepository::clone(url, &path)?;
        }

        let repo = Self::open(path)?;

        // Ensure HEAD is on master
        repo.repo.set_head(LOCAL_MASTER_REF)?;

        // Ensure repo is fresh
        repo.latest_commit()?.ensure_fresh()?;

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

    /// Read a file from the repository to a `String`
    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<RepoFile, Error> {
        let mut file = File::open(self.path.join(path.as_ref()))?;
        RepoFile::new(path.as_ref(), &mut file)
    }
}

/// Information about a commit to the Git repository
#[derive(Debug)]
pub struct CommitInfo {
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

        let target = head.target().ok_or_else(|| {
            err!(
                ErrorKind::Repo,
                "no ref target for: {}",
                repo.path.display()
            )
        })?;

        let mut commit_id = String::new();

        for byte in target.as_bytes().iter() {
            write!(commit_id, "{:02x}", byte)?;
        }

        let commit_object = repo.repo.find_object(target, Some(ObjectType::Commit))?;
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
            commit_id,
            ref_name,
            summary,
            #[cfg(feature = "chrono")]
            time,
        })
    }

    /// Determine if the repository is fresh or stale (i.e. has it recently been committed to)
    #[cfg(feature = "chrono")]
    pub fn ensure_fresh(&self) -> Result<(), Error> {
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
pub struct RepoFile {
    /// Relative location of file in repository
    path: PathBuf,

    /// Contents of file as a string
    body: String,
}

impl RepoFile {
    /// Create a RepoFile from a relative repo `Path` and a `File`
    pub(crate) fn new<P: Into<PathBuf>>(into_pathbuf: P, file: &mut File) -> Result<Self, Error> {
        let mut body = String::new();
        file.read_to_string(&mut body)?;
        Ok(Self {
            path: into_pathbuf.into(),
            body,
        })
    }
}

impl AsRef<str> for RepoFile {
    fn as_ref(&self) -> &str {
        self.body.as_ref()
    }
}
