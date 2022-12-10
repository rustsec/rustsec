//! Main entry point for the `rustsec-admin` CLI application

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use rustsec_admin::application::APPLICATION;

/// Boot the `rustsec-admin` CLI application
fn main() {
    // copied from `abscissa` code:
    // https://github.com/iqlusioninc/abscissa/blob/7c9ab7976145d2920a99ef00eba845efab3247a1/core/src/application.rs#L171-L177
    // so that we could inject wild::args() instead of std::env::args()
    // which provides glob expansion on Windows
    let args = std::env::args();
    abscissa_core::application::Application::run(&APPLICATION, args);
    std::process::exit(0);
}
