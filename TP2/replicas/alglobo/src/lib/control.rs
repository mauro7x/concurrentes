use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

use crate::{
    config::Config,
    constants::{CV_WAIT_ERROR, ELECTION_TIMEOUT, MUTEX_LOCK_ERROR},
    directory::Directory,
    protocols::election::{Message, COORDINATOR, ELECTION, NEW_MESSAGE, OK, PING, PONG},
    types::{BoxResult, Id, Shared},
};

// ----------------------------------------------------------------------------

pub struct Control {
    port: u16,
    id: Id,
    socket: UdpSocket,
    directory: Arc<Mutex<Directory>>,
    leader_id: Arc<Shared<Option<Id>>>,
    got_ok: Arc<Shared<bool>>,
}

impl Control {
    pub fn new() -> BoxResult<Self> {
        println!("[DEBUG] (Control) Creating Config...");
        let Config {
            port,
            directory_addr,
        } = Config::new()?;

        println!("[DEBUG] (Control) Creating Directory...");
        let directory = Directory::new(directory_addr)?;
        let id = directory.get_my_id();

        println!("[DEBUG] (Control) Creating and binding socket...");
        let mut ret = Control {
            port,
            id,
            socket: UdpSocket::bind(format!("0.0.0.0:{}", port))?,
            directory: Arc::new(Mutex::new(directory)),
            leader_id: Arc::new(Shared::new(None)),
            got_ok: Arc::new(Shared::new(false)),
        };

        println!("[DEBUG] (Control) Starting Receiver...");
        let mut clone = ret.clone()?;
        thread::spawn(move || clone.receiver());

        println!("[DEBUG] (Control) Finding current leader...");
        ret.init_leader()?;

        Ok(ret)
    }

    pub fn get_my_id(&self) -> BoxResult<Id> {
        let id = self.directory()?.get_my_id();

        Ok(id)
    }

    pub fn am_i_leader(&self) -> BoxResult<bool> {
        Ok(self.id == self.get_leader_id()?)
    }

    pub fn healthcheck_leader(&self) -> BoxResult<()> {
        while !self.am_i_leader()? {
            println!("[DEBUG] (Control) Updating my directory...");
            self.directory()?.update()?;
            println!("[DEBUG] (Control) Directory updated");

            println!("<Replica> Sending healthcheck...");
            // ping?
            println!("<Replica> Received response!");

            // sleep some time?
            std::thread::sleep(std::time::Duration::from_secs(3));
        }

        Ok(())
    }

    pub fn finish(&self) -> BoxResult<()> {
        // When there is no more work to do...
        if let Err(err) = self.directory()?.finish() {
            println!(
                "[WARN] (ID: {}) Error while finishing Directory: {}",
                self.id, err
            );
        };

        Ok(())
    }

    // Private

    fn clone(&self) -> BoxResult<Self> {
        let ret = Self {
            port: self.port,
            id: self.id,
            socket: self.socket.try_clone()?,
            directory: self.directory.clone(),
            leader_id: self.leader_id.clone(),
            got_ok: self.got_ok.clone(),
        };

        Ok(ret)
    }

    fn directory(&self) -> BoxResult<MutexGuard<Directory>> {
        Ok(self.directory.lock().map_err(|_| MUTEX_LOCK_ERROR)?)
    }

    fn init_leader(&mut self) -> BoxResult<()> {
        // Avoid unnecessary election:
        // -- if we have a leader in the network, get that ID silently
        // -- if not, initiate election

        // TODO: Ask to every node if there is a Leader
        // If not:
        println!("[INFO] (Control) No leader found, starting election");
        self.leader_election()?;

        Ok(())
    }

    fn find_new_leader(&mut self) -> BoxResult<()> {
        if self.is_finding_leader()? {
            return Ok(());
        };
        self.leader_election()?;

        Ok(())
    }

    fn leader_election(&mut self) -> BoxResult<()> {
        println!("[INFO] (ID: {}) (Control) Finding new leader", self.id);

        self.set_shared_value(self.got_ok.clone(), false)?;
        self.set_shared_value(self.leader_id.clone(), None)?;

        // Bully algorithm:
        if !self.send_election()? || !self.got_ok_within_timeout()? {
            self.make_me_leader()?;
        } else {
            self.get_leader_id()?;
        }

        Ok(())
    }

    fn got_ok_within_timeout(&self) -> BoxResult<bool> {
        let got_ok = *self
            .got_ok
            .cv
            .wait_timeout_while(
                self.got_ok.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)?,
                ELECTION_TIMEOUT,
                |got_it| !*got_it,
            )
            .map_err(|_| CV_WAIT_ERROR)?
            .0;

        Ok(got_ok)
    }

    fn send_election(&self) -> BoxResult<bool> {
        println!(
            "[INFO] (ID: {}) (Control) Broadcasting election message",
            self.id
        );
        let msg = self.msg_with_id(ELECTION);
        let mut directory = self.directory()?;
        let nodes = directory.get_updated_nodes()?;
        if nodes.is_empty() {
            return Ok(false);
        }

        for (id, ip) in nodes {
            if id < &self.id {
                self.socket.send_to(&msg, self.ip2addr(ip))?;
            };
        }

        Ok(true)
    }

    fn make_me_leader(&mut self) -> BoxResult<()> {
        println!("[INFO] (ID: {}) (Control) Announcing as leader", self.id);
        let msg = self.msg_with_id(COORDINATOR);
        let mut directory = self.directory()?;
        let nodes = directory.get_updated_nodes()?;

        for ip in nodes.values() {
            self.socket.send_to(&msg, self.ip2addr(ip))?;
        }

        self.new_leader(self.id)?;

        Ok(())
    }

    // Helpers

    fn new_leader(&self, id: Id) -> BoxResult<()> {
        *self.leader_id.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)? = Some(id);
        self.leader_id.cv.notify_all();

        Ok(())
    }

    fn msg_with_id(&self, header: u8) -> Vec<u8> {
        let mut msg = vec![header];
        msg.extend_from_slice(&self.id.to_le_bytes());
        msg
    }

    fn ip2addr(&self, ip: &Ipv4Addr) -> SocketAddr {
        SocketAddr::from(SocketAddrV4::new(*ip, self.port))
    }

    fn is_finding_leader(&self) -> BoxResult<bool> {
        Ok(self
            .leader_id
            .mutex
            .lock()
            .map_err(|_| MUTEX_LOCK_ERROR)?
            .is_none())
    }

    fn get_leader_id(&self) -> BoxResult<Id> {
        let id = self
            .leader_id
            .cv
            .wait_while(
                self.leader_id.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)?,
                |leader_id| leader_id.is_none(),
            )
            .map_err(|_| CV_WAIT_ERROR)?
            .ok_or("Leader ID is awkwardly none")?;

        Ok(id)
    }

    fn set_shared_value<V>(&mut self, shared: Arc<Shared<V>>, v: V) -> BoxResult<()> {
        *shared.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)? = v;

        Ok(())
    }

    // Receiver and handlers

    fn receiver(&mut self) {
        if let Err(err) = self.inner_receiver() {
            // TODO: Avoid this panic, propagate!
            panic!("[ERROR] (Control:Receiver) Crashed: {}", err)
        };
    }

    fn inner_receiver(&mut self) -> BoxResult<()> {
        let mut message: Message = NEW_MESSAGE;

        loop {
            let (_size, from) = self.socket.recv_from(&mut message)?;

            match message {
                [PING, _] => self.handle_ping(from)?,
                [opcode, id] => match opcode {
                    OK => self.handle_ok(from, id)?,
                    ELECTION => self.handle_election(from, id)?,
                    COORDINATOR => self.handle_coordinator(from, id)?,
                    _ => self.handle_invalid(from, id)?,
                },
            };
        }
    }

    fn handle_ping(&self, from: SocketAddr) -> BoxResult<()> {
        println!("[DEBUG] (Control:Receiver) PING from {}", from);
        self.socket.send_to(&PONG, from)?;

        Ok(())
    }

    fn handle_ok(&mut self, from: SocketAddr, id: Id) -> BoxResult<()> {
        println!("[DEBUG] (Control:Receiver) OK from {} (ID: {})", from, id);
        self.set_shared_value(self.got_ok.clone(), true)?;
        self.got_ok.cv.notify_all();

        Ok(())
    }

    fn handle_election(&self, from: SocketAddr, id: Id) -> BoxResult<()> {
        println!(
            "[DEBUG] (Control:Receiver) ELECTION from {} (ID: {})",
            from, id
        );
        if id > self.id {
            self.socket.send_to(&self.msg_with_id(OK), from)?;

            if !self.is_finding_leader()? {
                let mut cloned = self.clone()?;
                thread::spawn(move || {
                    if let Err(err) = cloned.find_new_leader() {
                        // TODO: Avoid this panic, propagate!
                        panic!("[ERROR] find_new_leader inside thread crashed: {}", err)
                    }
                    println!("Finished thread :P");
                });
            }
        }

        Ok(())
    }

    fn handle_coordinator(&mut self, from: SocketAddr, id: Id) -> BoxResult<()> {
        println!(
            "[DEBUG] (Control:Receiver) COORDINATOR from {} (ID: {})",
            from, id
        );
        self.new_leader(id)?;

        Ok(())
    }

    fn handle_invalid(&self, from: SocketAddr, id: Id) -> BoxResult<()> {
        println!(
            "[WARN] (Control:Receiver) Unknown from {} (ID: {}), ignoring",
            from, id
        );

        Ok(())
    }
}
