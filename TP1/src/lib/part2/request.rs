use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct RawRequest {
    pub origin: String,
    pub destiny: String,
    pub airline: String,
    pub package: bool,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub id: String,
    pub raw_request: RawRequest,
}
