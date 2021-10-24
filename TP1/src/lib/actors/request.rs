use actix::prelude::*;
use serde::Deserialize;

#[derive(Message, Debug, Deserialize)]
#[rtype(result = "()")]
pub struct IncommingRequest {
    pub origin: String,
    pub destiny: String,
    pub airline: String,
    pub package: bool,
}

#[derive(Message, Debug, Deserialize)]
#[rtype(result = "()")]
pub struct SystemRequest {
    pub id: u32,
    pub origin: String,
    pub destiny: String,
    pub package: bool,
}
