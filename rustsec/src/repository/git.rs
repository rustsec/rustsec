//! Git repository handling for the RustSec advisory DB

mod authentication;
mod commit;
#[cfg(feature = "osv-export")]
mod gitpath;
#[cfg(feature = "osv-export")]
mod modification_time;
mod repository;

pub use self::{authentication::with_authentication, commit::Commit, repository::Repository};

#[cfg(feature = "osv-export")]
pub use self::{gitpath::GitPath, modification_time::GitModificationTimes};

/// Location of the RustSec advisory database for crates.io
pub const DEFAULT_URL: &str = "https://github.com/RustSec/advisory-db.git";
