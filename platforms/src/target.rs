//! Target `cfg` attributes. Documented in the "Conditional compilation" section
//! of the Rust reference:
//!
//! <https://doc.rust-lang.org/reference/attributes.html#conditional-compilation>

mod arch;
mod env;
mod os;
mod pointerwidth;
mod endian;

pub use self::{
    arch::Arch,
    env::Env,
    os::OS,
    pointerwidth::PointerWidth,
    endian::Endian,
};
