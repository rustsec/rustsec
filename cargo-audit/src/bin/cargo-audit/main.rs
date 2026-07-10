//! Main entry point for `cargo audit`

#![forbid(unsafe_code)]

use cargo_audit::application::APP;

fn main() {
    abscissa_core::boot(&APP);
}
