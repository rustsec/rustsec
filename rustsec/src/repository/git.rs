//! Git repository handling for the RustSec advisory DB

mod authentication;
mod commit;
mod modification_time;
mod repository;

pub use self::{
    authentication::with_authentication, commit::Commit, modification_time::GitModificationTimes,
    repository::Repository,
};

/// Location of the RustSec advisory database for crates.io
pub const DEFAULT_URL: &str = "https://github.com/RustSec/advisory-db.git";
