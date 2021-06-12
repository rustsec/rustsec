use std::convert::TryInto;

use semver::VersionReq;
use semver::{Prerelease, Version};

use crate::advisory::versions::RawVersions;
use crate::advisory::Versions;
use crate::Error;

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
pub(crate) fn ranges_for_unvalidated_advisory(
    versions: &RawVersions,
) -> Result<Vec<OsvRange>, Error> {
    unaffected_to_osv_ranges(&versions.unaffected, &versions.patched)
}

/// Converts a list of unaffected ranges to a range of affected OSV ranges.
/// Since OSV ranges are a negation of the UNaffected ranges that RustSec stores,
/// the entire list has to be passed at once, both patched and unaffected ranges.
fn unaffected_to_osv_ranges(
    unaffected_req: &[VersionReq],
    patched_req: &[VersionReq],
) -> Result<Vec<OsvRange>, Error> {
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
                fail!(
                    crate::ErrorKind::BadParam,
                    format!("Overlapping version ranges: {} and {}", a, b)
                );
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
    v.build = Default::default(); // Clear any build metadata, it's not used to determine precedence
    if v.pre.is_empty() {
        // Not a pre-release.
        // Increment the last version and add "0" as pre-release specifier.
        // E.g. "1.2.3" is transformed to "1.2.4-0".
        // This seems to be the lowest possible version that's above 1.2.3 according to semver 2.0 spec
        v.patch += 1;
        v.pre = Prerelease::new("0").unwrap();
    } else {
        // It's a pre-release
        let mut parts: Vec<&str> = v.pre.split('.').collect();
        let last = parts.last().unwrap(); // we've already checked that it's a pre-release
        let incremented_last: String = if let Ok(numeric_version) = last.parse::<u64>() {
            // The last part is all numeric
            (numeric_version + 1).to_string()
        } else {
            // The last part is not a number, increment it lexicographically
            let mut replaced_a_char = false;
            let mut chars: Vec<char> = last.chars().collect();
            for c in chars.iter_mut().rev() {
                if let Some(next_char) = next_valid_char(*c) {
                    *c = next_char;
                    replaced_a_char = true;
                    break;
                }
            }
            // Edge case: all chars are at maximum, like 'zzzzzz'
            // We need to append the lowest valid character to the string
            if replaced_a_char == false {
                chars.push(ORDERED_VALID_CHARS[0]);
            }
            chars.into_iter().collect()
        };
        *parts.last_mut().unwrap() = &incremented_last;
        v.pre = Prerelease::new(&parts.join(".")).unwrap();
    }
    v
}

// // Lookup tables were generated using this code:
// fn main() {
//     let mut arr = Vec::new();
//     for c in 'a'..='z' {
//         arr.push(c);
//     }
//     for c in 'A'..='Z' {
//         arr.push(c);
//     }
//     for c in '0'..='9' {
//         arr.push(c);
//     }
//     arr.push('-');
//     arr.push('_');
//     arr.sort();
//     println!("{:?}", arr);
//     let mut look: Vec<Option<usize>> = vec![None; ('z' as usize)+1];
//     for (idx, valid_char) in arr.iter().enumerate() {
//         look[*valid_char as usize] = Some(idx);
//     }
//     println!("{:?}", look);
// }

const ORDERED_VALID_CHARS: [char; 64] = [
    '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
#[rustfmt::skip] //otherwise it makes this take up 123 lines
const INDEX_FOR_CHAR: [Option<usize>; 123] = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(0), None, None, Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), None, None, None, None, None, None, None, Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36), None, None, None, None, Some(37), None, Some(38), Some(39), Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), Some(63)];

fn next_valid_char(current: char) -> Option<char> {
    // panic instead of misbehaving silently if an invalid char is passed
    let idx = INDEX_FOR_CHAR[current as usize].unwrap();
    ORDERED_VALID_CHARS.get(idx + 1).copied()
}

#[cfg(test)]
mod tests {
    use super::increment;
    use semver::Version;

    #[test]
    fn increment_simple() {
        let input = Version::parse("1.2.3").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.4-0").unwrap();
        assert_eq!(expected, incremented);
    }

    #[test]
    fn increment_prerelease_numeric() {
        let input = Version::parse("1.2.3-9").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.3-10").unwrap();
        assert_eq!(expected, incremented);
    }

    #[test]
    fn increment_prerelease_numeric_multipart() {
        let input = Version::parse("1.2.3-4.5.6").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.3-4.5.7").unwrap();
        assert_eq!(expected, incremented);
    }

    #[test]
    fn increment_prerelease_alphanumeric() {
        let input = Version::parse("1.2.3-alpha1").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.3-alpha2").unwrap();
        assert_eq!(expected, incremented);
    }

    #[test]
    fn increment_prerelease_textual() {
        let input = Version::parse("1.2.3-alpha").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.3-alphb").unwrap();
        assert_eq!(expected, incremented);
    }

    #[test]
    fn increment_prerelease_textual_multipart() {
        let input = Version::parse("1.2.3-alpha.1.foo").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.3-alpha.1.fop").unwrap();
        assert_eq!(expected, incremented);
    }

    #[test]
    fn increment_prerelease_textual_weird() {
        let input = Version::parse("1.2.3-buzz").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.3-bvzz").unwrap();
        assert_eq!(expected, incremented);
    }

    #[test]
    fn increment_prerelease_textual_zzzz() {
        let input = Version::parse("1.2.3-zzzz").unwrap();
        let incremented = increment(&input);
        assert!(incremented > input);
        let expected = Version::parse("1.2.3-zzzz-").unwrap();
        assert_eq!(expected, incremented);
    }
}
