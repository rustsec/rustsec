//! Provides support for exporting to the interchange format defined by
//! https://github.com/google/osv
//!
//! We also use OSV-style ranges for version matching in RustSec crate
//! because it allows handling pre-releases correctly,
//! which `semver` crate does not allow doing directly.
//! See https://github.com/dtolnay/semver/issues/172

mod osv_advisory;
mod osv_range;
mod ranges_for_advisory;
mod unaffected_range;

pub use osv_advisory::OsvAdvisory;
pub use osv_range::OsvRange;
pub use ranges_for_advisory::ranges_for_advisory;
pub(crate) use ranges_for_advisory::ranges_for_unvalidated_advisory;
