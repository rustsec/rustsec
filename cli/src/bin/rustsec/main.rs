//! Main entry point for the `rustsec` CLI application

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use rustsec_cli::application::APPLICATION;

/// Boot the `rustsec` CLI application
fn main() {
    abscissa_core::boot(&APPLICATION);
}
