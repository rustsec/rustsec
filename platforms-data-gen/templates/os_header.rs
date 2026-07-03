//! Operating systems

use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, ser, de::Error as DeError, Deserialize, Serialize};

use crate::error::Error;

/// `target_os`: Operating system of the target.
///
/// This value is closely related to the second and third element
/// of the platform target triple, though it is not identical.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
