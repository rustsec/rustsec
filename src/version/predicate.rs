//! `VersionReq` predicates
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

use super::Version;
use crate::{Error, ErrorKind};
use semver::Identifier;
use std::{convert::TryFrom, fmt};

/// `VersionReq` predicates
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(super) struct Predicate {
    op: Op,
    major: u64,
    minor: Option<u64>,
    patch: Option<u64>,
    pre: Vec<Identifier>,
}

impl Predicate {
    pub fn matches(&self, ver: &Version) -> bool {
        match self.op {
            Op::Ex => self.is_exact(ver),
            Op::Gt => self.is_greater(ver),
            Op::GtEq => self.is_exact(ver) || self.is_greater(ver),
            Op::Lt => !self.is_exact(ver) && !self.is_greater(ver),
            Op::LtEq => !self.is_greater(ver),
            Op::Tilde => self.matches_tilde(ver),
            Op::Compatible => self.is_compatible(ver),
        }
    }

    fn is_exact(&self, ver: &Version) -> bool {
        if self.major != ver.major() {
            return false;
        }

        match self.minor {
            Some(minor) => {
                if minor != ver.minor() {
                    return false;
                }
            }
            None => return true,
        }

        match self.patch {
            Some(patch) => {
                if patch != ver.patch() {
                    return false;
                }
            }
            None => return true,
        }

        self.pre == ver.0.pre
    }

    fn is_greater(&self, ver: &Version) -> bool {
        if self.major != ver.major() {
            return ver.major() > self.major;
        }

        match self.minor {
            Some(minor) => {
                if minor != ver.minor() {
                    return ver.minor() > minor;
                }
            }
            None => return false,
        }

        match self.patch {
            Some(patch) => {
                if patch != ver.patch() {
                    return ver.patch() > patch;
                }
            }
            None => return false,
        }

        if self.pre.is_empty() {
            false
        } else {
            !ver.is_prerelease()
        }
    }

    // see https://www.npmjs.org/doc/misc/semver.html for behavior
    fn matches_tilde(&self, ver: &Version) -> bool {
        let minor = match self.minor {
            Some(n) => n,
            None => return self.major == ver.major(),
        };

        match self.patch {
            Some(patch) => {
                self.major == ver.major()
                    && minor == ver.minor()
                    && (ver.patch() > patch || (ver.patch() == patch))
            }
            None => self.major == ver.major() && minor == ver.minor(),
        }
    }

    // see https://www.npmjs.org/doc/misc/semver.html for behavior
    fn is_compatible(&self, ver: &Version) -> bool {
        if self.major != ver.major() {
            return false;
        }

        let minor = match self.minor {
            Some(n) => n,
            None => return self.major == ver.major(),
        };

        match self.patch {
            Some(patch) => {
                if self.major == 0 {
                    if minor == 0 {
                        ver.minor() == minor && ver.patch() == patch && self.pre_is_compatible(ver)
                    } else {
                        ver.minor() == minor
                            && (ver.patch() > patch
                                || (ver.patch() == patch && self.pre_is_compatible(ver)))
                    }
                } else {
                    ver.minor() > minor
                        || (ver.minor() == minor
                            && (ver.patch() > patch
                                || (ver.patch() == patch && self.pre_is_compatible(ver))))
                }
            }
            None => {
                if self.major == 0 {
                    ver.minor() == minor
                } else {
                    ver.minor() >= minor
                }
            }
        }
    }

    fn pre_is_compatible(&self, ver: &Version) -> bool {
        !ver.is_prerelease() || ver.0.pre >= self.pre
    }
}

impl TryFrom<semver_parser::range::Predicate> for Predicate {
    type Error = Error;

    fn try_from(other: semver_parser::range::Predicate) -> Result<Predicate, Error> {
        Ok(Predicate {
            op: Op::try_from(other.op)?,
            major: other.major,
            minor: other.minor,
            patch: other.patch,
            pre: other
                .pre
                .into_iter()
                .map(|id| match id {
                    semver_parser::version::Identifier::Numeric(n) => Identifier::Numeric(n),
                    semver_parser::version::Identifier::AlphaNumeric(s) => {
                        Identifier::AlphaNumeric(s)
                    }
                })
                .collect(),
        })
    }
}

impl fmt::Display for Predicate {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}{}", self.op, self.major)?;

        if let Some(v) = self.minor {
            write!(fmt, ".{}", v)?;
        }

        if let Some(v) = self.patch {
            write!(fmt, ".{}", v)?;
        }

        if !self.pre.is_empty() {
            write!(fmt, "-")?;
            for (i, x) in self.pre.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ".")?
                }
                write!(fmt, "{}", x)?;
            }
        }

        Ok(())
    }
}

/// `VersionReq` predicate ops
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Op {
    Ex,         // Exact
    Gt,         // Greater than
    GtEq,       // Greater than or equal to
    Lt,         // Less than
    LtEq,       // Less than or equal to
    Tilde,      // e.g. ~1.0.0
    Compatible, // compatible by definition of semver, indicated by ^
}

impl TryFrom<semver_parser::range::Op> for Op {
    type Error = Error;

    fn try_from(op: semver_parser::range::Op) -> Result<Self, Error> {
        use semver_parser::range;
        Ok(match op {
            range::Op::Ex => Op::Ex,
            range::Op::Gt => Op::Gt,
            range::Op::GtEq => Op::GtEq,
            range::Op::Lt => Op::Lt,
            range::Op::LtEq => Op::LtEq,
            range::Op::Tilde => Op::Tilde,
            range::Op::Compatible => Op::Compatible,
            range::Op::Wildcard(wildcard) => fail!(
                ErrorKind::Version,
                "wildcards in version requirements unsupported in advisories: '{:?}'",
                wildcard
            ),
        })
    }
}

impl fmt::Display for Op {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Op::Ex => write!(fmt, "= "),
            Op::Gt => write!(fmt, "> "),
            Op::GtEq => write!(fmt, ">= "),
            Op::Lt => write!(fmt, "< "),
            Op::LtEq => write!(fmt, "<= "),
            Op::Tilde => write!(fmt, "~"),
            Op::Compatible => write!(fmt, "^"),
        }
    }
}
