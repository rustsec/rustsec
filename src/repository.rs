//! Repository handling for the RustSec advisory DB

pub mod signature;

#[cfg(feature = "fetch")]
pub mod git;

#[cfg(feature = "fetch")]
pub use self::git::GitRepository;
