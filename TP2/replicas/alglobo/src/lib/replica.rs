use std::net::UdpSocket;

use crate::{
    config::Config,
    directory::Directory,
    types::{BoxResult, Id},
};

// ----------------------------------------------------------------------------

pub struct Replica {
    id: Id,
    socket: UdpSocket,
    directory: Directory,
}

impl Replica {
    pub fn new() -> BoxResult<Self> {
        println!("[DEBUG] Creating config...");
        let Config {
            port,
            directory_addr,
        } = Config::new()?;

        println!("[DEBUG] Establishing TCP connection with Directory...");
        let directory = Directory::new(directory_addr)?;
        let id = directory.get_my_id();

        println!("[DEBUG] Creating replica...");
        let ret = Replica {
            id,
            socket: UdpSocket::bind(format!("0.0.0.0:{}", port))?,
            directory,
        };

        println!("[DEBUG] Replica created successfully");
        Ok(ret)
    }

    pub fn run(&self) -> BoxResult<()> {
        println!("[INFO] Replica started");

        // TODO: Work
        println!("{:?}", self.directory);

        // When there is no more work to do...
        if let Err(err) = self.directory.finish() {
            println!("[WARN] Error while finishing Directory: {}", err);
        };

        println!("[INFO] Terminated gracefully");
        Ok(())
    }
}
