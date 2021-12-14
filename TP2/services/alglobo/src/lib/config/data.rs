use std::{env, net::SocketAddr};

use crate::{
    constants::env::{AIRLINE_HOST, BANK_HOST, DATA_PORT, HOTEL_HOST, SVC_PORT},
    types::common::BoxResult,
};

use super::utils::to_socket_addr;

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub bank_addr: SocketAddr,
    pub hotel_addr: SocketAddr,
    pub airline_addr: SocketAddr,
}

impl Config {
    pub fn new() -> BoxResult<Self> {
        let port: u16 = env::var(DATA_PORT)?.parse()?;
        let bank_addr = to_socket_addr(env::var(BANK_HOST)?, env::var(SVC_PORT)?.parse()?)?;
        let hotel_addr = to_socket_addr(env::var(HOTEL_HOST)?, env::var(SVC_PORT)?.parse()?)?;
        let airline_addr = to_socket_addr(env::var(AIRLINE_HOST)?, env::var(SVC_PORT)?.parse()?)?;

        Ok(Config {
            port,
            bank_addr,
            hotel_addr,
            airline_addr,
        })
    }
}
