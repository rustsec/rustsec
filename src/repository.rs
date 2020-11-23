//! Repository handling for the RustSec advisory DB

pub mod signature;

#[cfg(feature = "git")]
pub mod git;

#[cfg(feature = "git")]
pub use self::git::GitRepository;
