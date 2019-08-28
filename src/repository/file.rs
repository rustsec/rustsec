use crate::error::Error;
use std::path::{Path, PathBuf};

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
}
