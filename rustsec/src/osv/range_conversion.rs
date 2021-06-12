use std::convert::TryInto;

use semver::VersionReq;
use semver::{Prerelease, Version};

use crate::Error;
use crate::advisory::Versions;
use crate::advisory::versions::RawVersions;

use super::osv_range::OsvRange;
use super::unaffected_range::{Bound, UnaffectedRange};

/// Returns OSV ranges for all affected versions in the given advisory.
/// OSV ranges are `[start, end)` intervals, and anything included in them is affected.
/// Panics if the ranges are malformed or range specification syntax is not supported,
/// since that has been validated on deserialization.
pub fn ranges_for_advisory(versions: &Versions) -> Vec<OsvRange> {
    unaffected_to_osv_ranges(&versions.unaffected, &versions.patched).unwrap()
}

/// Returns OSV ranges for all affected versions in the given advisory.
/// OSV ranges are `[start, end)` intervals, and anything included in them is affected.
/// Errors if the ranges are malformed or range specification syntax is not supported.
pub(crate) fn ranges_for_unvalidated_advisory(versions: &RawVersions) -> Result<Vec<OsvRange>, Error> {
    unaffected_to_osv_ranges(&versions.unaffected, &versions.patched)
}

/// Converts a list of unaffected ranges to a range of affected OSV ranges.
/// Since OSV ranges are a negation of the UNaffected ranges that RustSec stores,
/// the entire list has to be passed at once, both patched and unaffected ranges.
fn unaffected_to_osv_ranges(unaffected_req: &[VersionReq], patched_req: &[VersionReq]) -> Result<Vec<OsvRange>, Error> {
    // Consolidate ranges for all versions that aren't affected
    let mut unaffected: Vec<UnaffectedRange> = Vec::new();
    for req in unaffected_req {
        unaffected.push(req.try_into()?);
    }
    for req in patched_req {
        unaffected.push(req.try_into()?);
    }

    // Edge case: no unaffected ranges specified. That means that ALL versions are affected.
    if unaffected.is_empty() {
        return Ok(vec![OsvRange {
            introduced: None,
            fixed: None,
        }]);
    }

    // Verify that the incoming ranges do not overlap. This is required for the correctness of the algoritm.
    // The current impl has quadratic complexity, but since we have like 4 ranges at most, this doesn't matter.
    // We can optimize this later if it starts showing up on profiles.
    for (idx, a) in unaffected[..unaffected.len() - 1].iter().enumerate() {
        for b in unaffected[idx + 1..].iter() {
            if a.overlaps(b) {
                fail!(crate::ErrorKind::BadParam,
                    format!("Overlapping version ranges: {} and {}", a, b));
            }
        }
    }

    // Now that we know that unaffected ranges don't overlap, we can simply order them by any of the bounds
    // and that will result in all ranges being ordered
    let mut unaffected = unaffected.to_vec();
    use std::cmp::Ordering;
    unaffected.sort_unstable_by(|a, b| {
        match (a.start().version(), b.start().version()) {
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
            (Some(v1), Some(v2)) => {
                assert!(v1 != v2); // should be already ruled out by overlap checks, but verify just in case
                v1.cmp(v2)
            }
        }
    });


    let mut result = Vec::new();

    // Handle the start bound of the first element, since it's not handled by the main loop
    match &unaffected.first().unwrap().start() {
        Bound::Unbounded => {} // Nothing to do
        Bound::Exclusive(v) => result.push(OsvRange {
            introduced: None,
            fixed: Some(increment(v)),
        }),
        Bound::Inclusive(v) => result.push(OsvRange {
            introduced: None,
            fixed: Some(v.clone()),
        }),
    }

    // Iterate over pairs of UnaffectedRange and turn the space between each pair into an OsvRange
    for r in unaffected.windows(2) {
        let start = match &r[0].end() {
            // ranges are ordered, so Unbounded can only appear in the first or last element, which are handled outside the loop
            Bound::Unbounded => unreachable!(),
            Bound::Exclusive(v) => v.clone(),
            Bound::Inclusive(v) => increment(v),
        };
        let end = match &r[1].start() {
            Bound::Unbounded => unreachable!(),
            Bound::Exclusive(v) => increment(v),
            Bound::Inclusive(v) => v.clone(),
        };
        result.push(OsvRange {
            introduced: Some(start),
            fixed: Some(end),
        });
    }

    // Handle the end bound of the last element, since it's not handled by the main loop
    match &unaffected.last().unwrap().end() {
        Bound::Unbounded => {} // Nothing to do
        Bound::Exclusive(v) => result.push(OsvRange {
            introduced: Some(v.clone()),
            fixed: None,
        }),
        Bound::Inclusive(v) => result.push(OsvRange {
            introduced: Some(increment(v)),
            fixed: None,
        }),
    }

    Ok(result)
}

/// Returns the lowest possible version greater than the input according to
/// [the SemVer 2.0 precedence rules](https://semver.org/#spec-item-11).
/// This is not the intutive "increment": this function returns a pre-release version!
/// E.g. "1.2.3" is transformed to "1.2.4-0".
fn increment(v: &Version) -> Version {
    let mut v = v.clone();
    if v.pre.is_empty() {
        // Not a pre-release.
        // Increment the last version and add "0" as pre-release specifier.
        // E.g. "1.2.3" is transformed to "1.2.4-0".
        // This seems to be the lowest possible version that's above 1.2.3 according to semver 2.0 spec
        v.build = Default::default(); // Clear any build metadata, it's not used to determine precedence
        v.patch += 1;
        v.pre = Prerelease::new("0").unwrap();
        v
    } else {
        let mut parts: Vec<&str> = v.pre.split('.').collect();
        let last = parts.last().unwrap(); // we've already checked that it's a pre-release
        if let Ok(numeric_version) = last.parse::<u64>() {
            let incremented_last = &(numeric_version+1).to_string();
            *parts.last_mut().unwrap() = incremented_last;
            v.pre = Prerelease::new(&parts.join(".")).unwrap();
            v
        } else {
            // implementing this would really suck,
            // and create rather unreadable specs too.
            // Like, 1.0-beta would turn into 1.0-betb, which is weird.
            // I think I'm just gonna emulate the inclusive range by
            // adding an affected standalone version...
            // Oh hell, I might need to increment the lower bound too,
            // if it originally was exclusive. Buck.
            // Looks like I'll have to implement it after all. TODO
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::osv::range_conversion::increment;

    #[test]
    fn increment_simple() {
        let input = semver::Version::parse("1.2.3").unwrap();
        let expected = semver::Version::parse("1.2.4-0").unwrap();
        assert_eq!(expected, increment(&input));
    }

    #[test]
    fn increment_prerelease_numeric() {
        let input = semver::Version::parse("1.2.3-9").unwrap();
        let expected = semver::Version::parse("1.2.3-10").unwrap();
        assert_eq!(expected, increment(&input));
    }
}