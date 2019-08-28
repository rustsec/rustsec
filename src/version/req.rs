//! Version requirements
//!
//! The version matching needed by RustSec is slightly different from the
//! logic needed by `semver` due to the handling of prerelease versions.
//!
//! See:
//! - https://github.com/RustSec/cargo-audit/issues/17
//! - https://github.com/RustSec/cargo-audit/issues/30
//! - https://github.com/steveklabnik/semver/issues/172

// Portions adapted from the `semver` crate.
// Copyright (c) 2016 Steve Klabnik

use super::{predicate::Predicate, Version};
use crate::{Error, ErrorKind};
use serde::{de, ser, Deserialize, Serialize};
use std::{convert::TryFrom, fmt, str::FromStr};

/// Version requirements
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct VersionReq {
    predicates: Vec<Predicate>,
}

impl VersionReq {
    /// Parse a version requirement from a string
    pub fn parse(input: &str) -> Result<Self, Error> {
        match semver_parser::range::parse(input) {
            Ok(req) => Self::try_from(req),
            Err(e) => fail!(ErrorKind::Version, "{}", e),
        }
    }

    /// Match the given `Version` against this `VersionReq`
    pub fn matches(&self, version: &Version) -> bool {
        if self.predicates.is_empty() {
            return true;
        }

        self.predicates.iter().all(|p| p.matches(version))
    }
}

impl FromStr for VersionReq {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Self::parse(s)
    }
}

impl fmt::Display for VersionReq {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.predicates.is_empty() {
            write!(fmt, "*")?;
        } else {
            for (i, ref pred) in self.predicates.iter().enumerate() {
                if i == 0 {
                    write!(fmt, "{}", pred)?;
                } else {
                    write!(fmt, ", {}", pred)?;
                }
            }
        }

        Ok(())
    }
}

impl TryFrom<semver_parser::range::VersionReq> for VersionReq {
    type Error = Error;

    fn try_from(other: semver_parser::range::VersionReq) -> Result<VersionReq, Error> {
        let mut predicates = Vec::with_capacity(other.predicates.len());

        for predicate in other.predicates.into_iter() {
            predicates.push(Predicate::try_from(predicate)?);
        }

        Ok(VersionReq { predicates })
    }
}

impl From<semver::VersionReq> for VersionReq {
    fn from(version_req: semver::VersionReq) -> VersionReq {
        Self::parse(&version_req.to_string()).unwrap()
    }
}

impl From<VersionReq> for semver::VersionReq {
    fn from(version_req: VersionReq) -> semver::VersionReq {
        semver::VersionReq::parse(&version_req.to_string()).unwrap()
    }
}

impl Serialize for VersionReq {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for VersionReq {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct VersionReqVisitor;

        impl<'de> de::Visitor<'de> for VersionReqVisitor {
            type Value = VersionReq;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a SemVer version requirement as a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                VersionReq::parse(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(VersionReqVisitor)
    }
}
