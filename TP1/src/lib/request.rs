use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub origin: String,
    pub destiny: String,
    pub airline: String,
    pub package: bool,
}
