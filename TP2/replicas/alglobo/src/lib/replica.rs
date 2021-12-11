use std::{net::UdpSocket, thread::sleep, time::Duration};

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

    pub fn run(&mut self) -> BoxResult<()> {
        println!("[INFO] (ID: {}) Replica started", self.id);
        self.directory.print();

        // TODO: Work
        {
            // TEMPORARY JUST FOR DEBUGGING
            println!("[DEBUG] Sleeping to simulate work...");
            sleep(Duration::from_secs(5));
            println!("[DEBUG] Awaken!");

            self.directory.update()?;
            self.directory.print();
        }

        // When there is no more work to do...
        if let Err(err) = self.directory.finish() {
            println!(
                "[WARN] (ID: {}) Error while finishing Directory: {}",
                self.id, err
            );
        };

        println!("[INFO] (ID: {}) Terminated gracefully", self.id);
        Ok(())
    }
}
