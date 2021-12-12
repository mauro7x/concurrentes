use std::{
    env,
    net::{SocketAddr, ToSocketAddrs},
};

use crate::{
    constants::env::{CTRL_PORT, DIRECTORY_HOST, DIRECTORY_PORT},
    types::common::BoxResult,
};

// ----------------------------------------------------------------------------

pub struct Config {
    pub port: u16,
    pub directory_addr: SocketAddr,
}

impl Config {
    pub fn new() -> BoxResult<Self> {
        let port: u16 = env::var(CTRL_PORT)?.parse()?;
        let directory_addr = Config::get_directory_addr()?;

        Ok(Config {
            port,
            directory_addr,
        })
    }

    fn get_directory_addr() -> BoxResult<SocketAddr> {
        let directory_host = env::var(DIRECTORY_HOST)?;
        let directory_port: u16 = env::var(DIRECTORY_PORT)?.parse()?;
        let directory_dns_query = format!("{}:{}", directory_host, directory_port);

        let directory_addr = directory_dns_query
            .to_socket_addrs()?
            .collect::<Vec<SocketAddr>>()
            .first()
            .ok_or("No IP address found for directory hostname")?
            .to_owned();

        Ok(directory_addr)
    }
}
