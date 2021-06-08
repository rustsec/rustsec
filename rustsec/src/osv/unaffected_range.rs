use semver::{Version, Op, Comparator, Prerelease};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Bound {
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
}

/// A range of unaffected versions, used by either `patched`
/// or `unaffected` fields in the security advisory.
/// Bounds may be inclusive or exclusive.
/// This is an intermediate representation private to this module.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnaffectedRange {
    pub start: Bound,
    pub end: Bound,
}

impl Default for UnaffectedRange {
    fn default() -> Self {
        UnaffectedRange {
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        }
    }
}

impl UnaffectedRange {
    /// Checks that `start <= end`
    /// TODO: fancy checked constructor for ranges or something,
    /// so we wouldn't have to call `.is_valid()` manually
    pub fn is_valid(&self) -> bool {
        let r = self;
        if r.start == Bound::Unbounded || r.end == Bound::Unbounded {
            true
        } else if r.start.version().unwrap() < r.end.version().unwrap() {
            true
        } else {
            match (&r.start, &r.end) {
                (Bound::Exclusive(v_start), Bound::Inclusive(v_end)) => v_start == v_end,
                (Bound::Inclusive(v_start), Bound::Exclusive(v_end)) => v_start == v_end,
                (Bound::Inclusive(v_start), Bound::Inclusive(v_end)) => v_start == v_end,
                (_, _) => false,
            }
        }
    }

    /// Requires ranges to be valid (i.e. `start <= end`) to work properly
    /// TODO: fancy checked constructor for ranges or something,
    /// so we wouldn't have to call `.is_valid()` manually
    pub fn overlaps(&self, other: &UnaffectedRange) -> bool {
        assert!(self.is_valid());
        assert!(other.is_valid());

        // range check for well-formed ranges is `(Start1 <= End2) && (Start2 <= End1)`
        // but it's complicated by our inclusive/exclusive bounds and unbounded ranges,
        // So we define a custom less_or_equal for this comparison

        fn less_or_equal(a: &Bound, b: &Bound) -> bool {
            match (a.version(), b.version()) {
                (Some(a_version), Some(b_version)) => {
                    if a_version > b_version {
                        false
                    } else if b_version == a_version {
                        match (a, b) {
                            (Bound::Inclusive(_), Bound::Inclusive(_)) => true,
                            // at least one of the fields is exclusive, and
                            // we've already checked that these fields are not unbounded,
                            // so they don't overlap
                            _ => false,
                        }
                    } else {
                        true
                    }
                }
                _ => true, // if one of the bounds is None
            }
        }

        less_or_equal(&self.start, &other.end) && less_or_equal(&other.start, &self.end)
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
impl From<&semver::VersionReq> for UnaffectedRange {
    fn from(input: &semver::VersionReq) -> Self {
        assert!(
            input.comparators.len() <= 2,
            "Unsupported version specification: too many comparators"
        );
        let mut result = UnaffectedRange::default();
        for comparator in &input.comparators {
            match comparator.op {
                Op::Exact => todo!(), // having a single exact unaffected version would be weird
                Op::Greater => {
                    assert!(
                        result.start == Bound::Unbounded,
                        "More than one lower bound in the same range!"
                    );
                    result.start = Bound::Exclusive(comp_to_ver(comparator));
                }
                Op::GreaterEq => {
                    assert!(
                        result.start == Bound::Unbounded,
                        "More than one lower bound in the same range!"
                    );
                    result.start = Bound::Inclusive(comp_to_ver(comparator));
                }
                Op::Less => {
                    assert!(
                        result.end == Bound::Unbounded,
                        "More than one upper bound in the same range!"
                    );
                    result.end = Bound::Exclusive(comp_to_ver(comparator));
                }
                Op::LessEq => {
                    assert!(
                        result.end == Bound::Unbounded,
                        "More than one upper bound in the same range!"
                    );
                    result.end = Bound::Inclusive(comp_to_ver(comparator));
                },
                Op::Caret => {
                    assert!(
                        input.comparators.len() == 1,
                        "Selectors that define both the upper and lower bound (e.g. '^1.0') must be alone in their range"
                    );
                    let start_version = comp_to_ver(comparator);
                    let mut end_version = if start_version.major == 0 {
                        Version::new(0,start_version.minor+1,0)
                    } else {
                        Version::new(&start_version.major+1, 0,0)
                    };
                    // -0 is the lowest possible prerelease.
                    // If we didn't append it, e.g. ^1.0.0 would match 2.0.0-pre
                    end_version.pre = Prerelease::new("0").unwrap();
                    result.start = Bound::Inclusive(start_version);
                    result.end = Bound::Exclusive(end_version);
                },
                _ => todo!(), // the struct is non-exhaustive, we have to do this
            }
        }
        assert!(result.is_valid());
        result
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
