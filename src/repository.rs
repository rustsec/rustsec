//! Repository handling for the RustSec advisory DB

pub mod signature;

#[cfg(feature = "fetch")]
pub mod git;

#[cfg(feature = "fetch")]
pub use self::git::GitRepository;

use chrono::{DateTime, Utc};
use signature::Signature;

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
