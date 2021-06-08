use semver::Version;

/// A range of affected versions.
/// If any of the bounds is unspecified, that means ALL versions
/// in that direction are affected.
///
/// This format is defined by https://github.com/google/osv
pub struct OsvRange {
    /// Inclusive
    pub start: Option<Version>,
    /// Exclusive
    pub end: Option<Version>,
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
