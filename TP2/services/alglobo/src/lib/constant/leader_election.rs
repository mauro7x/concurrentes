use std::time::Duration;

// ----------------------------------------------------------------------------

pub const GET_LEADER_TIMEOUT: Duration = Duration::from_secs(3);
pub const ELECTION_TIMEOUT: Duration = Duration::from_secs(3);
pub const REPLICA_SLEEP_TIME: Duration = Duration::from_secs(1);
pub const HEALTHCHECK_RETRIES: usize = 3;
/**
Keep in mind that the **total** healthcheck timeout will
be this value multiplied by the retries
*/
pub const HEALTHCHECK_TIMEOUT: Duration = Duration::from_secs(2);
