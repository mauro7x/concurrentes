use std::env;

use crate::{
    constants::{FAILURE_RATE, NAME, PORT, RESPONSE_TIME},
    types::common::BoxResult,
};

// ----------------------------------------------------------------------------

pub struct Config {
    pub name: String,
    pub port: u16,
    pub failure_rate: f64,
    pub response_time: u64,
}

impl Config {
    pub fn new() -> BoxResult<Self> {
        let name = env::var(NAME)?;
        let port = env::var(PORT)
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;
        let failure_rate = env::var(FAILURE_RATE)
            .unwrap_or_else(|_| "0.3".to_string())
            .parse()?;
        let response_time = env::var(RESPONSE_TIME)
            .unwrap_or_else(|_| "1000".to_string())
            .parse()?;

        Ok(Config {
            name,
            port,
            failure_rate,
            response_time,
        })
    }
}
