use std::env;

use crate::{constants::env::DATA_PORT, types::common::BoxResult};

// ----------------------------------------------------------------------------

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new() -> BoxResult<Self> {
        let port: u16 = env::var(DATA_PORT)?.parse()?;

        Ok(Config { port })
    }
}
