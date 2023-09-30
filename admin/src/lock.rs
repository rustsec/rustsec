//! Utiltity functions for file locking

use std::time::Duration;

use tame_index::{index::FileLock, utils::flock::LockOptions};

/// Acquires the Cargo package lock, or fails immediately
pub fn acquire_cargo_package_lock() -> Result<FileLock, tame_index::Error> {
    let lock_opts = LockOptions::cargo_package_lock(None)?.exclusive(false);
    acquire_lock(lock_opts, Duration::from_secs(0))
}

/// Acquires the provided lock with a speicifed timeout
pub fn acquire_lock(
    lock_opts: LockOptions<'_>,
    lock_timeout: Duration,
) -> Result<FileLock, tame_index::Error> {
    if lock_timeout == Duration::from_secs(0) {
        lock_opts.try_lock()
    } else {
        lock_opts.lock(|_| Some(lock_timeout))
    }
}
