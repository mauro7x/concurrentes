//! ControlPlane: Control logic for replica communication and leader election.

use std::{
    io::ErrorKind,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc, Mutex, MutexGuard,
    },
    thread,
};

use crate::{
    config::control::Config,
    constants::{
        errors::{CV_WAIT_ERROR, MUTEX_LOCK_ERROR},
        general::NONBLOCKING_POLLING_RATE,
        leader_election::{
            ELECTION_TIMEOUT, GET_LEADER_TIMEOUT, HEALTHCHECK_RETRIES, HEALTHCHECK_TIMEOUT,
            REPLICA_SLEEP_TIME,
        },
    },
    directory::Directory,
    protocol::election::{Message, ELECTION, GET_LEADER, LEADER, NEW_MESSAGE, OK, PING},
    thread_utils::{check_threads, safe_spawn},
    types::{
        common::{BoxResult, Id},
        control::{SafeThread, Shared},
    },
};

use log::*;

// ----------------------------------------------------------------------------

/// Control Plane is the entity that will control de Alglobo's replica logic and communication
/// with other replicas. An UDP socket is used underneath for network communication.
/// A separate entity called Directory will handle communication with Directory service.
/// Leader election and protocol with other services are handled in this logic layer.

pub struct ControlPlane {
    port: u16,
    id: Id,
    socket: UdpSocket,
    directory: Arc<Mutex<Directory>>,
    leader_id: Arc<Shared<Option<Id>>>,
    got_ok: Arc<Shared<bool>>,
    threads: Vec<SafeThread>,
    stopped: Arc<AtomicBool>,
}

impl ControlPlane {
    pub fn new() -> BoxResult<Self> {
        debug!("(ID: -) Creating Config...");
        let Config {
            port,
            directory_addr,
        } = Config::new()?;

        debug!("(ID: -) Creating Directory...");
        let directory = Directory::new(directory_addr)?;
        let id = directory.get_my_id();

        debug!("(ID: {}) Creating and binding socket...", id);
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))?;
        socket.set_nonblocking(true)?;

        let mut ret = Self {
            port,
            id,
            socket,
            directory: Arc::new(Mutex::new(directory)),
            leader_id: Arc::new(Shared::new(None)),
            got_ok: Arc::new(Shared::new(false)),
            threads: Vec::new(),
            stopped: Arc::new(AtomicBool::new(false)),
        };

        ret.init_leader()?;

        debug!("(ID: {}) Starting Receiver...", id);
        let cloned = ret.clone()?;
        safe_spawn(cloned, Self::receiver, &mut ret.threads)?;

        Ok(ret)
    }

    pub fn get_my_id(&self) -> BoxResult<Id> {
        let id = self.directory()?.get_my_id();

        Ok(id)
    }

    pub fn am_i_leader(&mut self) -> BoxResult<bool> {
        check_threads(&mut self.threads)?;

        Ok(self.id == self.get_leader_id()?)
    }

    pub fn healthcheck_leader(&mut self) -> BoxResult<()> {
        let healthcheck_socket = UdpSocket::bind("0.0.0.0:0")?;
        healthcheck_socket.set_read_timeout(Some(HEALTHCHECK_TIMEOUT))?;

        while !self.am_i_leader()? {
            info!("(ID: {}) Sending healthcheck to leader...", self.id);

            if self.is_leader_alive(&healthcheck_socket)? {
                info!("(ID: {}) Leader is alive!", self.id);
                thread::sleep(REPLICA_SLEEP_TIME);
            } else {
                info!("(ID: {}) Leader not responding, start election", self.id);
                self.find_new_leader()?;
            }
        }

        Ok(())
    }

    pub fn finish(&self) -> BoxResult<()> {
        // When there is no more work to do...
        if let Err(err) = self.directory()?.finish() {
            warn!("(ID: {}) Error while finishing Directory: {}", self.id, err);
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
            threads: Vec::new(),
            stopped: self.stopped.clone(),
        };

        Ok(ret)
    }

    fn directory(&self) -> BoxResult<MutexGuard<Directory>> {
        Ok(self.directory.lock().map_err(|_| MUTEX_LOCK_ERROR)?)
    }

    fn init_leader(&mut self) -> BoxResult<()> {
        info!("(ID: {}) Finding current leader...", self.id);
        let unshared_socket = UdpSocket::bind("0.0.0.0:0")?;

        {
            let msg = self.msg_with_id(GET_LEADER);
            let mut directory = self.directory()?;

            let nodes = directory.get_updated_nodes()?;
            if nodes.is_empty() {
                // I am the only node in the network,
                // make myself leader without asking
                info!("(ID: {}) No more nodes found, starting as leader", self.id);
                self.set_new_leader(self.id)?;
                return Ok(());
            }

            for ip in nodes.values() {
                unshared_socket.send_to(&msg, self.ip2addr(ip))?;
            }
        }

        let mut message: Message = NEW_MESSAGE;
        unshared_socket.set_read_timeout(Some(GET_LEADER_TIMEOUT))?;
        if unshared_socket.recv_from(&mut message).is_ok() {
            let [opcode, id] = message;
            match opcode {
                LEADER => {
                    info!("(ID: {}) Found leader with ID: {}", self.id, id);
                    self.set_new_leader(id)?;
                }
                _ => return Err("Unknown response to GET_LEADER received".into()),
            }
        } else {
            warn!(
                "(ID: {}) Nobody responded, announcing myself as leader",
                self.id
            );
            self.make_me_leader()?;
        }

        Ok(())
    }

    fn find_new_leader(&mut self) -> BoxResult<()> {
        check_threads(&mut self.threads)?;

        if self.is_finding_leader()? {
            return Ok(());
        };

        info!("(ID: {}) Finding new leader", self.id);
        self.set_shared_value(self.got_ok.clone(), false)?;
        self.set_shared_value(self.leader_id.clone(), None)?;

        // Bully algorithm:
        if !self.send_election()? || !self.got_ok_within_timeout()? {
            check_threads(&mut self.threads)?;
            self.make_me_leader()?;
        } else {
            self.get_leader_id()?;
        }

        Ok(())
    }

    // AtomicBool is no useful because we need to
    // wait for this with a CV, so we need a MutexGuard
    #[allow(clippy::mutex_atomic)]
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
        info!("(ID: {}) Broadcasting election message", self.id);

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
        info!("(ID: {}) Announcing as leader", self.id);
        let msg = self.msg_with_id(LEADER);
        let mut directory = self.directory()?;
        let nodes = directory.get_updated_nodes()?;

        for ip in nodes.values() {
            self.socket.send_to(&msg, self.ip2addr(ip))?;
        }

        self.set_new_leader(self.id)?;

        Ok(())
    }

    fn is_leader_alive(&mut self, socket: &UdpSocket) -> BoxResult<bool> {
        let mut attempts = 0;
        let mut recv_buf = [0; 1];

        loop {
            check_threads(&mut self.threads)?;

            match self.get_leader_addr()? {
                Some(leader_addr) => {
                    socket.send_to(&[PING], leader_addr)?;
                    if socket.recv_from(&mut recv_buf).is_ok() {
                        return Ok(true);
                    } else {
                        if attempts == HEALTHCHECK_RETRIES {
                            return Ok(false);
                        };

                        attempts += 1;
                    }
                }
                None => return Ok(false),
            };
        }
    }

    // Helpers

    fn set_new_leader(&self, id: Id) -> BoxResult<()> {
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

    fn get_leader_addr(&self) -> BoxResult<Option<SocketAddr>> {
        let leader_id = self.get_leader_id()?;
        let mut directory = self.directory()?;
        directory.update()?;

        Ok(directory
            .get_node_addr(leader_id)?
            .map(|ip| self.ip2addr(ip)))
    }

    fn set_shared_value<V>(&mut self, shared: Arc<Shared<V>>, v: V) -> BoxResult<()> {
        *shared.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)? = v;

        Ok(())
    }

    // Receiver and handlers

    fn receiver(&mut self) -> BoxResult<()> {
        while !self.stopped.load(Relaxed) {

            self.recv_msg()?;
        }

        Ok(())
    }

    fn recv_msg(&mut self) -> BoxResult<()> {
        let mut message: Message = NEW_MESSAGE;

        match self.socket.recv_from(&mut message) {
            Ok((_size, from)) => match message {
                [PING, _] => self.handle_ping(from),
                [opcode, id] => match opcode {
                    OK => self.handle_ok(from, id),
                    ELECTION => self.handle_election(from, id),
                    LEADER => self.handle_leader(from, id),
                    GET_LEADER => self.handle_get_leader(from, id),
                    _ => self.handle_invalid(from, id),
                },
            },
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {
                    thread::sleep(NONBLOCKING_POLLING_RATE);
                    Ok(())
                }
                _ => Err(err.into()),
            },
        }
    }

    fn handle_ping(&self, from: SocketAddr) -> BoxResult<()> {
        debug!("(ID: {}) (Control:Receiver) PING from {}", self.id, from);
        self.socket.send_to(&[OK], from)?;

        Ok(())
    }

    fn handle_ok(&mut self, from: SocketAddr, id: Id) -> BoxResult<()> {
        debug!(
            "(ID: {}) (Control:Receiver) OK from {} (ID: {})",
            self.id, from, id
        );
        self.set_shared_value(self.got_ok.clone(), true)?;
        self.got_ok.cv.notify_all();

        Ok(())
    }

    fn handle_election(&mut self, from: SocketAddr, id: Id) -> BoxResult<()> {
        debug!(
            "(ID: {}) (Control:Receiver) ELECTION from {} (ID: {})",
            self.id, from, id
        );
        if id > self.id {
            self.socket.send_to(&self.msg_with_id(OK), from)?;

            if !self.is_finding_leader()? {
                let cloned = self.clone()?;
                safe_spawn(cloned, Self::find_new_leader, &mut self.threads)?;
            }
        }

        Ok(())
    }

    fn handle_leader(&mut self, from: SocketAddr, id: Id) -> BoxResult<()> {
        debug!(
            "(ID: {}) (Control:Receiver) LEADER from {} (ID: {})",
            self.id, from, id
        );
        self.set_new_leader(id)?;

        Ok(())
    }

    fn handle_get_leader(&mut self, from: SocketAddr, id: Id) -> BoxResult<()> {
        debug!(
            "(ID: {}) (Control:Receiver) GET_LEADER from {} (ID: {})",
            self.id, from, id
        );
        if self.am_i_leader()? {
            self.socket.send_to(&self.msg_with_id(LEADER), from)?;
        }

        Ok(())
    }

    fn handle_invalid(&self, from: SocketAddr, id: Id) -> BoxResult<()> {
        warn!(
            "(ID: {}) (Control:Receiver) Unknown from {} (ID: {}), ignoring",
            self.id, from, id
        );

        Ok(())
    }
}

impl Drop for ControlPlane {
    fn drop(&mut self) {
        debug!("(ID: {}) Destroying...", self.id);
        self.stopped.store(true, Relaxed);
        while let Some(thread) = self.threads.pop() {
            thread.joiner.join().expect("Error joining threads");
        }
        debug!("(ID: {}) Destroyed successfully", self.id);
    }
}
