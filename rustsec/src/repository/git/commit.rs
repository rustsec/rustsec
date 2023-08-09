//! Commits to the advisory DB git repository

use crate::{
    error::{Error, ErrorKind},
    repository::{git::Repository, signature::Signature},
};
use std::time::{Duration, SystemTime};

/// Number of days after which the repo will be considered stale
/// (90 days)
const STALE_AFTER: Duration = Duration::from_secs(90 * 86400);

/// Information about a commit to the Git repository
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
#[derive(Debug)]
pub struct Commit {
    /// ID (i.e. SHA-1 hash) of the latest commit
    pub commit_id: gix::ObjectId,

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
        let commit = repo
            .repo
            .head_commit()
            .map_err(|err| format_err!(ErrorKind::Repo, "unable to locate head commit: {}", err))?;

        // Since we are pulling multiple pieces from the commit it's better to do this once
        let cref = commit.decode().map_err(|err| {
            format_err!(
                ErrorKind::Repo,
                "unable to decode commit information: {}",
                err
            )
        })?;

        let commit_id = commit.id;
        let timestamp = crate::repository::git::gix_time_to_time(cref.committer.time);
        let author = {
            let sig = cref.author();
            format!("{} <{}>", sig.name, sig.email)
        };

        let summary = cref.message_summary().to_string();

        if summary.is_empty() {
            return Err(format_err!(
                ErrorKind::Repo,
                "no commit summary for {}",
                commit_id
            ));
        }

        let (signature, signed_data) = if let Some(sig) = cref.extra_headers().pgp_signature() {
            // Note this is inefficient as gix doesn't yet support signature extraction natively.
            // TODO: convert this to native methods once https://github.com/Byron/gitoxide/pull/973 ships in a stable release.
            let signed_data = {
                let mut commit_without_signature = cref.clone();
                let pos = commit_without_signature
                    .extra_headers
                    .iter()
                    .position(|eh| eh.0 == "gpgsig")
                    .unwrap();
                commit_without_signature.extra_headers.remove(pos);

                let mut signed_data = Vec::new();
                use gix::objs::WriteTo;
                commit_without_signature.write_to(&mut signed_data)?;
                signed_data
            };

            (Some(Signature::from_bytes(sig)?), Some(signed_data))
        } else {
            (None, None)
        };

        Ok(Self {
            commit_id,
            author,
            summary,
            timestamp,
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
        let repo = &repo.repo;
        let workdir = repo.work_dir().ok_or_else(|| {
            format_err!(ErrorKind::Repo, "unable to checkout, repository is bare")
        })?;

        // This is a manual implementation of resetting a repository to a given commit,
        // since `gix` doesn't support this natively yet.
        // TODO: switch to `gix` implementation once it becomes available.
        let root_tree = repo
            .find_object(self.commit_id)
            .map_err(|err| format_err!(ErrorKind::Repo, "unable to locate commit: {}", err))?
            .peel_to_tree()
            .map_err(|err| format_err!(ErrorKind::Repo, "unable to peel to tree: {}", err))?
            .id;

        let index = gix::index::State::from_tree(&root_tree, |oid, buf| {
            repo.objects.find_tree_iter(oid, buf).ok()
        })
        .map_err(|err| {
            format_err!(
                ErrorKind::Repo,
                "failed to create index from tree '{}': {}",
                root_tree,
                err
            )
        })?;

        use gix::prelude::FindExt;
        let mut index = gix::index::File::from_state(index, repo.index_path());

        let opts = gix::worktree::checkout::Options {
            destination_is_initially_empty: false,
            overwrite_existing: true,
            ..Default::default()
        };

        gix::worktree::checkout(
            &mut index,
            workdir,
            {
                let objects = repo.objects.clone().into_arc()?;
                move |oid, buf| objects.find_blob(oid, buf)
            },
            &mut gix::progress::Discard,
            &mut gix::progress::Discard,
            &gix::interrupt::IS_INTERRUPTED,
            opts,
        )
        .map_err(|err| format_err!(ErrorKind::Repo, "failed to checkout: {}", err))?;

        index
            .write(Default::default())
            .map_err(|err| format_err!(ErrorKind::Repo, "failed to write index: {}", err))?;

        Ok(())
    }
}
