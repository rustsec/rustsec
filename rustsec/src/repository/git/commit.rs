//! Commits to the advisory DB git repository

use crate::{
    error::{Error, ErrorKind},
    repository::{
        git::{self, Repository},
        signature::Signature,
    },
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    pub timestamp: time::OffsetDateTime,

    /// Signature on the commit (mandatory for Repository::fetch)
    // TODO: actually verify signatures
    pub signature: Option<Signature>,

    /// Signed data to verify along with this commit
    signed_data: Option<Vec<u8>>,
}

impl Commit {
    /// Get information about HEAD
    pub(crate) fn from_repo_head(repo: &Repository) -> Result<Self, Error> {
        let find_remote_head = || -> Result<(gix::Commit<'_>, time::OffsetDateTime), Error> {
            const CANDIDATE_REFS: &[&str] = &[
                "FETCH_HEAD",  /* the location with the most-recent updates, as written by gix/us */
                "origin/HEAD", /* typical refspecs update this symbolic ref to point to the actual remote ref with the fetched commit */
                "origin/main", /* for good measure, resolve this branch by hand in case origin/HEAD is broken */
                "HEAD", /* last resort, this would only be needed for a fresh clone via git/git2 */
            ];
            let mut candidates: Vec<_> = CANDIDATE_REFS
                .iter()
                .enumerate()
                .filter_map(|(i, refname)| {
                    let ref_id = repo
                        .repo
                        .find_reference(*refname)
                        .ok()?
                        .into_fully_peeled_id()
                        .ok()?;

                    let commit = ref_id.object().ok()?.try_into_commit().ok()?;
                    let commit_time = commit.time().ok()?;

                    Some((i, commit, commit_time))
                })
                .collect();

            // Sort from oldest to newest, the last one will be the best reference
            // we could reasonably locate, and since we are on second resolution,
            // prefer the ordering of candidates if times are equal (can happen during eg testing)
            //
            // This allows FETCH_HEAD to be authoritative, unless one of the other
            // references is more up to date, which can occur in (at least) 2 scenarios:
            //
            // 1. The repo is a fresh clone by cargo either via git or libgit2,
            // neither of which write FETCH_HEAD during clone
            // 2. A fetch was performed by an external crate/program to
            // ourselves that didn't update FETCH_HEAD
            candidates.sort_by(|a, b| match a.2.seconds.cmp(&b.2.seconds) {
                std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                o => o,
            });

            // get the most recent commit, the one with most time passed since unix epoch.
            let best = candidates.last().ok_or_else(|| {
                format_err!(ErrorKind::Repo, "unable to find a suitable HEAD commit")
            })?;

            // In case we used FETCH_HEAD, use the mtime of the file itself, as
            // it will be updated on every (successful) fetch and is a true
            // time of the last update, rather than just the time of the last
            // remote commit
            let commit_time = if best.0 == 0 {
                std::fs::metadata(repo.repo.path().join("FETCH_HEAD"))
                    .ok()
                    .and_then(|md| md.modified().ok().map(|t| t.into()))
            } else {
                None
            };

            let commit_time = commit_time.unwrap_or_else(|| git::gix_time_to_time(best.2));

            Ok((best.1, commit_time))
        };

        let (commit, commit_time) = find_remote_head()?;

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

        Ok(Self {
            commit_id,
            author,
            summary,
            timestamp: commit_time,
            signature,
            signed_data,
        })
    }

    /// Finds the most appropriate head commit for the repo
    fn find_head(repo: &Repository) -> Result<(&'static str,), Error> {}

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
