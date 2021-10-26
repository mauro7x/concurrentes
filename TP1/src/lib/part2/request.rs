//! Request entities.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawRequest {
    pub origin: String,
    pub destiny: String,
    pub airline: String,
    pub package: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: String,
    pub raw_request: RawRequest,
}
