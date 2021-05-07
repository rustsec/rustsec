//! Target `cfg` attributes. Documented in the "Conditional compilation" section
//! of the Rust reference:
//!
//! <https://doc.rust-lang.org/reference/attributes.html#conditional-compilation>

mod arch;
mod env;
mod os;

pub use self::{
    arch::{Arch, TARGET_ARCH},
    env::{Env, TARGET_ENV},
    os::{OS, TARGET_OS},
};
