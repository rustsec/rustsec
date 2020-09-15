//! Repository handling for the RustSec advisory DB

#[cfg(feature = "fetch")]
pub mod git;

pub mod signature;

use crate::Error;
use signature::Signature;
use std::path::PathBuf;

use chrono::{DateTime, Utc};

/// Information about a commit to the Git repository
#[derive(Debug)]
pub struct Commit {
    /// ID (i.e. SHA-1 hash) of the latest commit
    pub commit_id: String,

    /// Information about the author of a commit
    pub author: String,

    /// Summary message for the commit
    pub summary: String,

    /// Commit time in number of seconds since the UNIX epoch
    pub time: DateTime<Utc>,

    /// Signature on the commit (mandatory for Repository::fetch)
    // TODO: actually verify signatures
    pub signature: Option<Signature>,

    /// Signed data to verify along with this commit
    signed_data: Option<Vec<u8>>,
}

/// Repository for a Rust advisory DB.
pub trait Repository {
    /// Get information about the latest commit to the repo
    fn latest_commit(&self) -> Result<Commit, Error>;

    /// Paths to all advisories located in the database
    fn advisories(&self) -> Result<Vec<PathBuf>, Error>;
}
