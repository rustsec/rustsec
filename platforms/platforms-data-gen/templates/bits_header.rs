//! Pointer width of the target architecture

use crate::error::Error;
use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, ser, de::Error as DeError, Deserialize, Serialize};

/// `target_pointer_width`: Size of native pointer types (`usize`, `isize`) in bits
///
/// 64 bits for modern desktops and phones, 32-bits for older devices, 16 bits for certain microcontrollers
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
