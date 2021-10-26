//! Request entities.

use serde::{Deserialize, Serialize};

/// Incomming parsed request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawRequest {
    pub origin: String,
    pub destiny: String,
    pub airline: String,
    pub package: bool,
}

/// Entity that is used to keep track of petition status.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: String,
    pub start_time: i64,
    pub raw_request: RawRequest,
}
