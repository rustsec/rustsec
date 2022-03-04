//! Endianness

use crate::error::Error;
use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};

/// `target_endian`: [Endianness](https://en.wikipedia.org/wiki/Endianness) of the target.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
