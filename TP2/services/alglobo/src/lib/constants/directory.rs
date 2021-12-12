use std::time::Duration;

// ----------------------------------------------------------------------------

pub const CONNECTION_MAX_ATTEMPTS: usize = 3;
pub const CONNECTION_RETRY_TIME: Duration = Duration::from_secs(1);
