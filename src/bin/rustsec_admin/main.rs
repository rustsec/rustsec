//! Main entry point for Admin

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use rustsec_admin::application::APPLICATION;

/// Boot Admin
fn main() {
    abscissa_core::boot(&APPLICATION);
}
