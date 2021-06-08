use std::path::Path;

use crate::{Error, ErrorKind};

use super::Repository;

/// A path *relative to the root of the git repository*
/// that's guaranteed to exist in the given repository.
pub struct GitPath<'a> {
    repo: &'a Repository,
    path: &'a Path,
}

impl<'a> GitPath<'a> {
    pub fn new(repo: &'a Repository, path: &'a Path) -> Result<Self, Error> {
        // Validate that the path is relative for better feedback to API users
        if path.has_root() {
            return Err(format_err!(
                ErrorKind::BadParam,
                "{} is not a relative path",
                path.display()
            ));
        }
        let commit_id = repo.repo.refname_to_id("HEAD")?;
        let commit = repo.repo.find_commit(commit_id)?;
        commit.tree()?.get_path(path)?;
        Ok(GitPath { repo, path })
    }
}