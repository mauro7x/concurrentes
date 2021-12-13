use std::{env, net::SocketAddr};

use crate::{
    constants::env::{CTRL_PORT, DIRECTORY_HOST, DIRECTORY_PORT},
    types::common::BoxResult,
};

use super::utils::to_socket_addr;

// ----------------------------------------------------------------------------

pub struct Config {
    pub port: u16,
    pub directory_addr: SocketAddr,
}

impl Config {
    pub fn new() -> BoxResult<Self> {
        let port: u16 = env::var(CTRL_PORT)?.parse()?;
        let directory_addr = to_socket_addr(
            env::var(DIRECTORY_HOST)?,
            env::var(DIRECTORY_PORT)?.parse()?,
        )?;

        Ok(Config {
            port,
            directory_addr,
        })
    }
}
