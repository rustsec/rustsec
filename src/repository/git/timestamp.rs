//! Git timestamps

use serde::{Deserialize, Serialize};
pub use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Number of days after which the repo will be considered stale
/// (90 days)
pub const STALE_AFTER: Duration = Duration::from_secs(90 * 86400);

/// Git timestamps
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Timestamp {
    /// Inner timestamp value
    #[serde(with = "humantime_serde")]
    inner: SystemTime,
}

impl Timestamp {
    /// Create a new timestamp from a Unix time in seconds
    pub fn new(unix_secs: u64) -> Self {
        Timestamp {
            inner: UNIX_EPOCH + Duration::from_secs(unix_secs),
        }
    }

    /// Is this timestamp "fresh" as in the database has been updated recently
    /// (i.e. 90 days, per the `STALE_AFTER` constant)
    pub fn is_fresh(self) -> bool {
        self.inner > SystemTime::now().checked_sub(STALE_AFTER).unwrap()
    }
}

impl From<SystemTime> for Timestamp {
    fn from(system_time: SystemTime) -> Timestamp {
        Timestamp { inner: system_time }
    }
}

impl From<Timestamp> for SystemTime {
    fn from(timestamp: Timestamp) -> SystemTime {
        timestamp.inner
    }
}
