use std::time::Duration;

// ----------------------------------------------------------------------------

pub const RESTART_TIME: Duration = Duration::from_secs(10);
pub const DIRECTORY_CONNECTION_MAX_ATTEMPTS: usize = 3;
pub const DIRECTORY_CONNECTION_RETRY_TIME: Duration = Duration::from_secs(1);
