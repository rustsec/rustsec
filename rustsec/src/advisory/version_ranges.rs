//! Transforms version requirements as provided by the `semver` crate
//! into a bunch of `[start; end)` ranges where the starting version
//! is always inclusive, and the end version is always exclusive.
//!
//! This is used for exporting to OSV format.
//! This also allows handling pre-releases correctly,
//! which `semver` crate does not allow doing directly.
//! See https://github.com/steveklabnik/semver/issues/172

use semver::{Comparator, Op, Version};

/// Returns OSV ranges for all affected versions in the given advisory.
/// OSV ranges are `[start, end)` intervals, and anything included in them is affected.
pub fn ranges_for_advisory(versions: &super::Versions) -> Vec<OsvRange> {
    let mut unaffected: Vec<UnaffectedRange> = Vec::new();
    for req in &versions.unaffected {
        unaffected.push(req.into());
    }
    for req in &versions.patched {
        unaffected.push(req.into());
    }
    unaffected_to_osv_ranges(&unaffected)
}

/// A range of affected versions.
/// If any of the bounds is unspecified, that means ALL versions
/// in that direction are affected.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OsvRange {
    /// Inclusive
    start: Option<Version>,
    /// Exclusive
    end: Option<Version>,
}

impl OsvRange {
    /// Returns true if the given version is affected
    pub fn contains(&self, v: &Version) -> bool {
        (match &self.start {
            None => true,
            Some(start_v) => v >= start_v,
        }) && (match &self.end {
            None => true,
            Some(end_v) => v < end_v,
        })
    }
}

/// A range of unaffected versions, used by either `patched`
/// or `unaffected` fields in the security advisory.
/// Bounds may be inclusive or exclusive.
/// This is an intermediate representation private to this module.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct UnaffectedRange {
    start: Bound,
    end: Bound,
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
    fn is_valid(&self) -> bool {
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
    fn overlaps(&self, other: &UnaffectedRange) -> bool {
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Bound {
    Unbounded,
    Exclusive(Version),
    Inclusive(Version),
}

impl Bound {
    /// Returns just the version, ignoring whether the bound is inclusive or exclusive
    fn version(&self) -> Option<&Version> {
        match &self {
            Bound::Unbounded => None,
            Bound::Exclusive(v) => Some(v),
            Bound::Inclusive(v) => Some(v),
        }
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
        //println!("Comparators in `from`: {:?}", &input.comparators);
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
                    let end_version = if start_version.major == 0 {
                        Version::new(0,start_version.minor+1,0)
                    } else {
                        Version::new(&start_version.major+1, 0,0)
                    };
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

/// Converts a list of unaffected ranges to a range of affected OSV ranges.
/// Since OSV ranges are a negation of the UNaffected ranges that RustSec stores,
/// the entire list has to be passed at once, both patched and unaffected ranges.
fn unaffected_to_osv_ranges(unaffected: &[UnaffectedRange]) -> Vec<OsvRange> {
    // Verify that all incoming ranges are valid. TODO: a checked constructor or something.
    unaffected.iter().for_each(|r| assert!(r.is_valid()));

    // Edge case: no unaffected ranges specified. That means that ALL versions are affected.
    if unaffected.is_empty() {
        return vec![OsvRange{start: None, end: None}];
    }

    //println!("Comparators in `unaff_to_osv`: {:?}", &unaffected);

    // Verify that the incoming ranges do not overlap. This is required for the correctness of the algoritm.
    // The current impl has quadratic complexity, but since we have like 4 ranges at most, this doesn't matter.
    // We can optimize this later if it starts showing up on profiles.
    for (idx, a) in unaffected[..unaffected.len() - 1].iter().enumerate() {
        for b in unaffected[idx+1..].iter() {
            //println!("comparing {:?} to {:?}", a, b);
            assert!(!a.overlaps(b));
        }
    }

    // Now that we know that unaffected ranges don't overlap, we can simply order them by any of the bounds
    // and that will result in all ranges being ordered
    let mut unaffected = unaffected.to_vec();
    use std::cmp::Ordering;
    unaffected.sort_unstable_by(|a, b| {
        match (a.start.version(), b.start.version()) {
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
            (Some(v1), Some(v2)) => {
                assert!(v1 != v2); // should be already ruled out by overlap checks, but verify just in case
                v1.cmp(v2)
            }
        }
    });

    // Unhandled edge case in increment logic: two ranges back to back, one inclusive other exclusive
    // This does not cause overlap in UnaffectedRange representation, but would result in overlapping OSV ranges.
    // This can be fixed by coalescing such ranges, and it's just an O(n) pass!
    // TODO: coalesce such ranges

    let mut result = Vec::new();

    // Handle the start bound of the first element, since it's not handled by the main loop
    match &unaffected.first().unwrap().start {
        Bound::Unbounded => {} // Nothing to do
        Bound::Exclusive(v) => result.push(OsvRange {
            start: None,
            end: Some(increment(v)),
        }),
        Bound::Inclusive(v) => result.push(OsvRange {
            start: None,
            end: Some(v.clone()),
        }),
    }

    // Iterate over pairs of UnaffectedRange and turn the space between each pair into an OsvRange
    for r in unaffected.windows(2) {
        let start = match &r[0].end {
            // ranges are ordered, so Unbounded can only appear in the first or last element, which are handled outside the loop
            Bound::Unbounded => unreachable!(),
            Bound::Exclusive(v) => v.clone(),
            Bound::Inclusive(v) => increment(v),
        };
        let end = match &r[1].start {
            Bound::Unbounded => unreachable!(),
            Bound::Exclusive(v) => increment(v),
            Bound::Inclusive(v) => v.clone(),
        };
        result.push(OsvRange {
            start: Some(start),
            end: Some(end),
        });
    }

    // Handle the end bound of the last element, since it's not handled by the main loop
    match &unaffected.last().unwrap().end {
        Bound::Unbounded => {} // Nothing to do
        Bound::Exclusive(v) => result.push(OsvRange {
            start: Some(v.clone()),
            end: None,
        }),
        Bound::Inclusive(v) => result.push(OsvRange {
            start: Some(increment(v)),
            end: None,
        }),
    }

    result
}

fn increment(v: &Version) -> Version {
    if v.pre.is_empty() {
        // Not a pre-release.
        // Increment the last version and add "0" as pre-release specifier.
        // E.g. "1.2.3" is transformed to "1.2.4-0".
        // This seems to be the lowest possible version that's above 1.2.3 according to semver 2.0 spec
        let mut v = v.clone();
        v.build = Default::default(); // Clear any build metadata, it's not used to determine precedence
        v.patch += 1;
        // add pre-release version in string form because I really don't want to mess with private types in semver crate
        let mut serialized = v.to_string();
        serialized.push_str("-0");
        Version::parse(&serialized).unwrap()
    } else {
        todo!() //TODO: increment pre-release version
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
