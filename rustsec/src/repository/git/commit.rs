//! Commits to the advisory DB git repository

use crate::{
    error::{Error, ErrorKind},
    repository::{git::Repository, signature::Signature},
};
use std::{
    ops::Deref,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Number of days after which the repo will be considered stale
/// (90 days)
const STALE_AFTER: Duration = Duration::from_secs(90 * 86400);

/// Information about a commit to the Git repository
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
#[derive(Debug)]
pub struct Commit {
    /// ID (i.e. SHA-1 hash) of the latest commit
    pub commit_id: String,

    /// Information about the author of a commit
    pub author: String,

    /// Summary message for the commit
    pub summary: String,

    /// Commit time in number of seconds since the UNIX epoch
    pub timestamp: SystemTime,

    /// Signature on the commit (mandatory for Repository::fetch)
    // TODO: actually verify signatures
    pub signature: Option<Signature>,

    /// Signed data to verify along with this commit
    signed_data: Option<Vec<u8>>,
}

impl Commit {
    /// Get information about HEAD
    pub(crate) fn from_repo_head(repo: &Repository) -> Result<Self, Error> {
        let head = repo.repo.head()?;

        let oid = head.target().ok_or_else(|| {
            format_err!(
                ErrorKind::Repo,
                "no ref target for: {}",
                repo.path.display()
            )
        })?;

        let commit_id = oid.to_string();
        let commit_object = repo.repo.find_object(oid, Some(git2::ObjectType::Commit))?;
        let commit = commit_object.as_commit().unwrap();
        let author = commit.author().to_string();

        let summary = commit
            .summary()
            .ok_or_else(|| format_err!(ErrorKind::Repo, "no commit summary for {}", commit_id))?
            .to_owned();

        let (signature, signed_data) = match repo.repo.extract_signature(&oid, None) {
            Ok((ref sig, ref data)) => {
                (Some(Signature::from_bytes(sig)?), Some(data.deref().into()))
            }
            _ => (None, None),
        };

        let time = UNIX_EPOCH + Duration::from_secs(commit.time().seconds() as u64);

        Ok(Commit {
            commit_id,
            author,
            summary,
            timestamp: time,
            signature,
            signed_data,
        })
    }

    /// Is the commit timestamp "fresh" as in the database has been updated
    /// recently? (i.e. 90 days, per the `STALE_AFTER` constant)
    pub fn is_fresh(&self) -> bool {
        self.timestamp > SystemTime::now().checked_sub(STALE_AFTER).unwrap()
    }

    /// Get the raw bytes to be verified when verifying a commit signature
    pub fn raw_signed_bytes(&self) -> Option<&[u8]> {
        self.signed_data.as_ref().map(|bytes| bytes.as_ref())
    }

    /// Reset the repository's state to match this commit
    pub(crate) fn reset(&self, repo: &Repository) -> Result<(), Error> {
        let commit_object = repo.repo.find_object(
            git2::Oid::from_str(&self.commit_id).unwrap(),
            Some(git2::ObjectType::Commit),
        )?;

        // Reset the state of the repository to the latest commit
        repo.repo
            .reset(&commit_object, git2::ResetType::Hard, None)?;

        Ok(())
    }
}
