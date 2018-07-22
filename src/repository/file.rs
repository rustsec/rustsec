use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use error::Error;

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
