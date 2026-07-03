//! Rust architectures

use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, ser, de::Error as DeError, Deserialize, Serialize};

use crate::error::Error;

/// `target_arch`: Target CPU architecture
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
