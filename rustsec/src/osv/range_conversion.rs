use semver::{Prerelease, Version};

use super::osv_range::OsvRange;
use super::unaffected_range::{Bound, UnaffectedRange};

/// Returns OSV ranges for all affected versions in the given advisory.
/// OSV ranges are `[start, end)` intervals, and anything included in them is affected.
pub fn ranges_for_advisory(versions: &crate::advisory::Versions) -> Vec<OsvRange> {
    let mut unaffected: Vec<UnaffectedRange> = Vec::new();
    for req in &versions.unaffected {
        unaffected.push(req.into());
    }
    for req in &versions.patched {
        unaffected.push(req.into());
    }
    unaffected_to_osv_ranges(&unaffected)
}

/// Converts a list of unaffected ranges to a range of affected OSV ranges.
/// Since OSV ranges are a negation of the UNaffected ranges that RustSec stores,
/// the entire list has to be passed at once, both patched and unaffected ranges.
fn unaffected_to_osv_ranges(unaffected: &[UnaffectedRange]) -> Vec<OsvRange> {
    // Verify that all incoming ranges are valid. TODO: a checked constructor or something.
    unaffected.iter().for_each(|r| assert!(r.is_valid()));

    // Edge case: no unaffected ranges specified. That means that ALL versions are affected.
    if unaffected.is_empty() {
        return vec![OsvRange {
            start: None,
            end: None,
        }];
    }

    // Verify that the incoming ranges do not overlap. This is required for the correctness of the algoritm.
    // The current impl has quadratic complexity, but since we have like 4 ranges at most, this doesn't matter.
    // We can optimize this later if it starts showing up on profiles.
    for (idx, a) in unaffected[..unaffected.len() - 1].iter().enumerate() {
        for b in unaffected[idx + 1..].iter() {
            // TODO: better message because it might be shown to users
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
        // add pre-release version in string form because these types are private in semver crate
        v.pre = Prerelease::new("0").unwrap();
        v
    } else {
        todo!() //TODO: increment pre-release version
    }
}
