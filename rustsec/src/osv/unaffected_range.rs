//! This is an intermediate representation used for converting from
//! Cargo-style version selectors (`>=`, `^`, `<`, etc) to OSV rang es.
//! It is an implementation detail and is not exported outside OSV module.

use std::convert::TryFrom;

use semver::{Comparator, Op, Prerelease, Version};

use crate::Error;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Bound {
    Unbounded,
    Exclusive(Version),
    Inclusive(Version),
}

impl Bound {
    /// Returns just the version, ignoring whether the bound is inclusive or exclusive
    pub fn version(&self) -> Option<&Version> {
        match &self {
            Bound::Unbounded => None,
            Bound::Exclusive(v) => Some(v),
            Bound::Inclusive(v) => Some(v),
        }
    }

    /// We don't actually need full-blown `Ord`
    fn less_or_equal(&self, other: &Bound) -> bool {
        let start = self;
        let end = other;
        if start == &Bound::Unbounded || end == &Bound::Unbounded {
            true
        } else if start.version().unwrap() < end.version().unwrap() {
            true
        } else {
            match (&start, &end) {
                (Bound::Inclusive(v_start), Bound::Inclusive(v_end)) => v_start == v_end,
                (_, _) => false,
            }
        }
    }
}

/// A range of unaffected versions, used by either `patched`
/// or `unaffected` fields in the security advisory.
/// Bounds may be inclusive or exclusive.
/// `start` is guaranteed to be less than or equal to `end`.
/// If `start == end`, both bounds must be inclusive.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct UnaffectedRange {
    start: Bound,
    end: Bound,
}

impl UnaffectedRange {
    pub fn new(start: Bound, end: Bound) -> Result<Self, Error> {
        if start.less_or_equal(&end) {
            Ok(UnaffectedRange { start, end })
        } else {
            Err(format_err!(
                crate::ErrorKind::BadParam,
                "Invalid range: start must be <= end; if equal, both bounds must be inclusive"
            ))
        }
    }

    pub fn start(&self) -> &Bound {
        &self.start
    }

    pub fn end(&self) -> &Bound {
        &self.end
    }

    pub fn overlaps(&self, other: &UnaffectedRange) -> bool {
        // range check for well-formed ranges is `(Start1 <= End2) && (Start2 <= End1)`
        self.start.less_or_equal(&other.end) && other.start.less_or_equal(&self.end)
    }
}

// To keep the algorithm simple, we make several assumptions:
// 1. There are at most two version boundaries per `VersionReq`.
//    This means that stuff like `>= 1.0 < 1.5 || >= 2.0 || 2.5`
//    is not supported. RustSec format uses a list of ranges for that instead...
//    Which is probably not a great idea in retrospect.
// 2. There is at most one upper and at most one lower bound in each range.
//    Stuff like `>= 1.0, >= 2.0` is nonsense.
// 3. If the requirement is "1.0" or "^1.0" that defines both the lower and upper bound,
//    it is the only one in its range.
// If any of those assumptions are violated, it will panic.
// This is fine for the advisory database as of May 2021.
impl TryFrom<&semver::VersionReq> for UnaffectedRange {
    type Error = Error;

    fn try_from(input: &semver::VersionReq) -> Result<Self, Self::Error> {
        assert!(
            input.comparators.len() <= 2,
            "Unsupported version specification: too many comparators"
        );
        let mut start = Bound::Unbounded;
        let mut end = Bound::Unbounded;
        for comparator in &input.comparators {
            match comparator.op {
                Op::Exact => todo!(), // having a single exact unaffected version would be weird
                Op::Greater => {
                    assert!(
                        start == Bound::Unbounded,
                        "More than one lower bound in the same range!"
                    );
                    start = Bound::Exclusive(comp_to_ver(comparator));
                }
                Op::GreaterEq => {
                    assert!(
                        start == Bound::Unbounded,
                        "More than one lower bound in the same range!"
                    );
                    start = Bound::Inclusive(comp_to_ver(comparator));
                }
                Op::Less => {
                    assert!(
                        end == Bound::Unbounded,
                        "More than one upper bound in the same range!"
                    );
                    end = Bound::Exclusive(comp_to_ver(comparator));
                }
                Op::LessEq => {
                    assert!(
                        end == Bound::Unbounded,
                        "More than one upper bound in the same range!"
                    );
                    end = Bound::Inclusive(comp_to_ver(comparator));
                }
                Op::Caret => {
                    assert!(
                        input.comparators.len() == 1,
                        "Selectors that define both the upper and lower bound (e.g. '^1.0') must be alone in their range"
                    );
                    let start_version = comp_to_ver(comparator);
                    let mut end_version = if start_version.major == 0 {
                        Version::new(0, start_version.minor + 1, 0)
                    } else {
                        Version::new(&start_version.major + 1, 0, 0)
                    };
                    // -0 is the lowest possible prerelease.
                    // If we didn't append it, e.g. ^1.0.0 would match 2.0.0-pre
                    end_version.pre = Prerelease::new("0").unwrap();
                    start = Bound::Inclusive(start_version);
                    end = Bound::Exclusive(end_version);
                }
                _ => todo!(), // the struct is non-exhaustive, we have to do this
            }
        }
        // TODO: validate, don't unwrap
        Ok(UnaffectedRange::new(start, end).unwrap())
    }
}

/// Strips comparison operators from a Comparator and turns it into a Version.
/// Would have been better implemented by `into` but these are foreign types
fn comp_to_ver(c: &Comparator) -> Version {
    Version {
        major: c.major,
        minor: c.minor.unwrap_or(0),
        patch: c.patch.unwrap_or(0),
        pre: c.pre.clone(),
        build: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_unbounded() {
        let range1 = UnaffectedRange {
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        };
        let range2 = UnaffectedRange {
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        };
        assert!(range1.overlaps(&range2));
        assert!(range2.overlaps(&range1));
    }

    #[test]
    fn barely_not_overlapping() {
        let range1 = UnaffectedRange {
            start: Bound::Inclusive(Version::parse("1.2.3").unwrap()),
            end: Bound::Unbounded,
        };
        let range2 = UnaffectedRange {
            start: Bound::Unbounded,
            end: Bound::Exclusive(Version::parse("1.2.3").unwrap()),
        };
        assert!(!range1.overlaps(&range2));
        assert!(!range2.overlaps(&range1));
    }

    #[test]
    fn barely_overlapping() {
        let range1 = UnaffectedRange {
            start: Bound::Inclusive(Version::parse("1.2.3").unwrap()),
            end: Bound::Unbounded,
        };
        let range2 = UnaffectedRange {
            start: Bound::Unbounded,
            end: Bound::Inclusive(Version::parse("1.2.3").unwrap()),
        };
        assert!(range1.overlaps(&range2));
        assert!(range2.overlaps(&range1));
    }

    #[test]
    fn clearly_not_overlapping() {
        let range1 = UnaffectedRange {
            start: Bound::Inclusive(Version::parse("0.1.0").unwrap()),
            end: Bound::Inclusive(Version::parse("0.3.0").unwrap()),
        };
        let range2 = UnaffectedRange {
            start: Bound::Inclusive(Version::parse("1.1.0").unwrap()),
            end: Bound::Inclusive(Version::parse("1.3.0").unwrap()),
        };
        assert!(!range1.overlaps(&range2));
        assert!(!range2.overlaps(&range1));
    }

    #[test]
    fn clearly_overlapping() {
        let range1 = UnaffectedRange {
            start: Bound::Inclusive(Version::parse("0.1.0").unwrap()),
            end: Bound::Inclusive(Version::parse("1.1.0").unwrap()),
        };
        let range2 = UnaffectedRange {
            start: Bound::Inclusive(Version::parse("0.2.0").unwrap()),
            end: Bound::Inclusive(Version::parse("1.3.0").unwrap()),
        };
        assert!(range1.overlaps(&range2));
        assert!(range2.overlaps(&range1));
    }
}
