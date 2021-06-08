//! Provides support for exporting to the interchange format defined by
//! https://github.com/google/osv
//! 
//! We also use OSV-style ranges for version matching in RustSec crate
//! because it allows handling pre-releases correctly,
//! which `semver` crate does not allow doing directly.
//! See https://github.com/dtolnay/semver/issues/172

// These are implementation details and are not exported outside OSV module
mod unaffected_range;
mod osv_range;
mod range_conversion;

pub use osv_range::OsvRange;
pub use range_conversion::ranges_for_advisory;