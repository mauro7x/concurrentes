use std::env;

use crate::{
    constants::env::{AIRLINE_ADDR, BANK_ADDR, DATA_PORT, HOTEL_ADDR, PAYMENTS_CSV},
    types::common::BoxResult,
};

// ----------------------------------------------------------------------------

pub struct Config {
    pub port: String,
    pub bank_addr: String,
    pub hotel_addr: String,
    pub airline_addr: String,
    pub payments_file_path: String,
}

impl Config {
    pub fn new() -> BoxResult<Self> {
        Ok(Config {
            port: env::var(DATA_PORT)?,
            bank_addr: env::var(BANK_ADDR)?,
            hotel_addr: env::var(HOTEL_ADDR)?,
            airline_addr: env::var(AIRLINE_ADDR)?,
            payments_file_path: env::var(PAYMENTS_CSV)?,
        })
    }
}
