//! Provides support for exporting to the interchange format defined by
//! https://github.com/google/osv
//!
//! We also use OSV-style ranges for version matching in RustSec crate
//! because it allows handling pre-releases correctly,
//! which `semver` crate does not allow doing directly.
//! See https://github.com/dtolnay/semver/issues/172

#[cfg(feature = "osv-export")]
mod osv_advisory;
#[cfg(feature = "osv-export")]
pub use osv_advisory::OsvAdvisory;

// The rest are enabled unconditionally because the OSV range format
// is used for determining whether a given version is affected or not

mod osv_range;
mod ranges_for_advisory;
mod unaffected_range;

pub use osv_range::OsvRange;
pub use ranges_for_advisory::ranges_for_advisory;
pub(crate) use ranges_for_advisory::ranges_for_unvalidated_advisory;
