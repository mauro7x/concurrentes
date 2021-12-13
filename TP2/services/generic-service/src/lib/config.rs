use std::env;

use crate::{
    constants::{FAILURE_RATE, NAME, PORT},
    types::common::BoxResult,
};

// ----------------------------------------------------------------------------

pub struct Config {
    pub name: String,
    pub port: u16,
    pub failure_rate: f64,
}

impl Config {
    pub fn new() -> BoxResult<Self> {
        let name = env::var(NAME)?;
        let port = env::var(PORT).unwrap_or("3000".to_string()).parse()?;
        let failure_rate = env::var(FAILURE_RATE)
            .unwrap_or("0.3".to_string())
            .parse()?;

        Ok(Config {
            name,
            port,
            failure_rate,
        })
    }
}
