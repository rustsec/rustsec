//! Git repository handling for the RustSec advisory DB

mod authentication;
mod commit;
mod repository;
mod modification_time;

pub use self::{authentication::with_authentication, commit::Commit, repository::Repository};

/// Location of the RustSec advisory database for crates.io
pub const DEFAULT_URL: &str = "https://github.com/RustSec/advisory-db.git";
