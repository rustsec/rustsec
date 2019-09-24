//! Main entry point for the `rustsec-admin` CLI application

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use rustsec_admin::application::APPLICATION;

/// Boot the `rustsec-admin` CLI application
fn main() {
    abscissa_core::boot(&APPLICATION);
}
