//! Rust target environments

use crate::error::Error;
use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, ser, de::Error as DeError, Deserialize, Serialize};

/// `target_env`: target environment that disambiguates the target platform by ABI / libc.
///
/// This value is closely related to the fourth element of the platform target triple,
/// though it is not identical. For example, embedded ABIs such as `gnueabihf` will simply
/// define `target_env` as `"gnu"` (i.e. `target::Env::GNU`)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
