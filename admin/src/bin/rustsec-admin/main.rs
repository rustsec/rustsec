//! Main entry point for the `rustsec-admin` CLI application

#![forbid(unsafe_code)]

use rustsec_admin::application::APPLICATION;

/// Boot the `rustsec-admin` CLI application
fn main() {
    abscissa_core::boot(&APPLICATION);
}
