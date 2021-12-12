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
pub const GET_LEADER_TIMEOUT: Duration = Duration::from_secs(3);
pub const ELECTION_TIMEOUT: Duration = Duration::from_secs(3);
pub const REPLICA_SLEEP_TIME: Duration = Duration::from_secs(1);
pub const HEALTHCHECK_RETRIES: usize = 3;
/**
Keep in mind that the **total** healthcheck timeout will
be this value multiplied by the retries
*/
pub const HEALTHCHECK_TIMEOUT: Duration = Duration::from_secs(2);

// Env vars
pub const PORT: &str = "PORT";
pub const DIRECTORY_HOST: &str = "DIRECTORY_HOST";
pub const DIRECTORY_PORT: &str = "DIRECTORY_PORT";
