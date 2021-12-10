use std::{
    env,
    error::Error,
    net::{SocketAddr, ToSocketAddrs},
};

pub struct Config {
    pub port: u32,
    pub directory_addr: SocketAddr,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let port: u32 = env::var("PORT")?.parse()?;
        let directory_addr = Config::get_directory_addr()?;

        Ok(Config {
            port,
            directory_addr,
        })
    }

    fn get_directory_addr() -> Result<SocketAddr, Box<dyn Error>> {
        let directory_host = env::var("DIRECTORY_HOST")?;
        let directory_port: u16 = env::var("DIRECTORY_PORT")?.parse()?;
        let directory_dns_query = format!("{}:{}", directory_host, directory_port);

        let directory_addr = directory_dns_query
            .to_socket_addrs()?
            .collect::<Vec<SocketAddr>>()
            .first()
            .unwrap()
            .to_owned();

        Ok(directory_addr)
    }
}
