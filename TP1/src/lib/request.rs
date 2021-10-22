use serde::Deserialize;

pub type RequestDuration = (Request, i64);

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    pub origin: String,
    pub destiny: String,
    pub airline: String,
    pub package: bool,
}
