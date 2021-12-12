use std::time::Duration;

// ----------------------------------------------------------------------------

// General
pub const RESTART_TIME: Duration = Duration::from_secs(10);

// Directory connection establishment
pub const DIRECTORY_CONNECTION_MAX_ATTEMPTS: usize = 3;
pub const DIRECTORY_CONNECTION_RETRY_TIME: Duration = Duration::from_secs(1);

// Errors
pub const MUTEX_LOCK_ERROR: &str = "Failed to lock Mutex";
pub const CV_WAIT_ERROR: &str = "Failed to wait CV";

// Leader election
pub const ELECTION_TIMEOUT: Duration = Duration::from_secs(3);
