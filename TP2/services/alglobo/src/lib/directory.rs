use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    mem::size_of,
    net::{Ipv4Addr, SocketAddr, TcpStream},
    thread::sleep,
};

use crate::{
    constants::directory::{CONNECTION_MAX_ATTEMPTS, CONNECTION_RETRY_TIME},
    protocol::directory::{RecvOpcode, ACCEPTED, DEAD, EOB, FINISHED, NEW, REGISTER, REJECTED},
    types::{
        common::{BoxResult, Id},
        control::{Id2Ip, Ip2Id, Node},
    },
};

// ----------------------------------------------------------------------------

pub struct Directory {
    id: Id,
    addr: SocketAddr,
    stream: TcpStream,
    id2ip: Id2Ip,
    ip2id: Ip2Id,
}

impl Directory {
    pub fn new(addr: SocketAddr) -> BoxResult<Self> {
        let mut stream = Directory::connect_with_attemps(addr, CONNECTION_MAX_ATTEMPTS)?;
        let (id, id2ip, ip2id) = Directory::register(&mut stream)?;
        stream.set_nonblocking(true)?;

        let ret = Directory {
            id,
            addr,
            stream,
            id2ip,
            ip2id,
        };
        Ok(ret)
    }

    pub fn finish(&self) -> BoxResult<()> {
        let mut stream = TcpStream::connect(self.addr)?;
        stream.write_all(&FINISHED)?;

        Ok(())
    }

    pub fn get_my_id(&self) -> Id {
        self.id
    }

    pub fn update(&mut self) -> BoxResult<()> {
        let mut opcode: RecvOpcode = [0; 1];

        match self.stream.read_exact(&mut opcode) {
            Ok(_) => {
                let id = Directory::recv_id(&mut self.stream)?;
                let ip = Directory::recv_ip(&mut self.stream)?;
                self.inner_update(opcode, id, ip)?;

                self.update()
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => Ok(()),
                _ => Err(err.into()),
            },
        }
    }

    pub fn get_updated_nodes(&mut self) -> BoxResult<&Id2Ip> {
        self.update()?;
        Ok(&self.id2ip)
    }

    pub fn get_node_addr(&mut self, id: Id) -> BoxResult<Option<&Ipv4Addr>> {
        self.update()?;
        Ok(self.id2ip.get(&id))
    }

    // Private

    fn inner_update(&mut self, opcode: RecvOpcode, id: Id, ip: Ipv4Addr) -> BoxResult<()> {
        match opcode {
            NEW => {
                if let Some(old_id) = self.ip2id.insert(ip, id) {
                    self.id2ip.remove(&old_id);
                };
                if let Some(old_ip) = self.id2ip.insert(id, ip) {
                    println!("[WARN] (ID: {}) This block should never be reached, since the Directory should not allow for IDs collisions", self.id);
                    self.ip2id.remove(&old_ip);
                }
            }
            DEAD => {
                self.id2ip.remove(&id);
                self.ip2id.remove(&ip);
            }
            _ => {
                println!(
                    "[ERROR] (ID: {}) Received unexpected opcode from Directory, aborting",
                    self.id
                );
                return Err("Unexpected opcode from Directory".into());
            }
        };

        Ok(())
    }

    // Abstract

    fn connect_with_attemps(addr: SocketAddr, max_attemps: usize) -> BoxResult<TcpStream> {
        let mut attempts = 0;

        loop {
            match TcpStream::connect(addr) {
                Ok(stream) => return Ok(stream),
                Err(err) => {
                    if attempts == max_attemps {
                        println!("[ERROR] (ID: -) Max directory connection attemps, aborting");
                        return Err(err.into());
                    };

                    attempts += 1;
                    sleep(CONNECTION_RETRY_TIME);
                }
            }
        }
    }

    fn register(stream: &mut TcpStream) -> BoxResult<(Id, Id2Ip, Ip2Id)> {
        // Start registration process by sending OPCODE
        stream.write_all(&REGISTER)?;

        // Receive fields according to protocol
        Directory::recv_accepted(stream)?;
        let id = Directory::recv_id(stream)?;
        let nodes = Directory::recv_nodes(stream)?;

        // Parse nodes to HashMaps
        let (id2ip, ip2id) = Directory::parse_nodes(nodes);

        Ok((id, id2ip, ip2id))
    }

    fn recv_accepted(stream: &mut TcpStream) -> BoxResult<()> {
        let mut buf: [u8; 1] = [0; 1];
        stream.read_exact(&mut buf)?;

        match buf {
            ACCEPTED => {}
            REJECTED => {
                println!("[WARN] (ID: -) Register rejected because directory is full, aborting");
                return Err("Rejected: Directory full".into());
            }
            [_] => {
                println!("[ERROR] (ID: -) Received unknown message from Directory, aborting");
                return Err("Unknown message from Directory".into());
            }
        }

        Ok(())
    }

    fn recv_id(stream: &mut TcpStream) -> BoxResult<Id> {
        let mut id_buf: [u8; size_of::<Id>()] = [0; size_of::<Id>()];
        stream.read_exact(&mut id_buf)?;

        Ok(id_buf[0])
    }

    fn recv_ip(stream: &mut TcpStream) -> BoxResult<Ipv4Addr> {
        let mut ip_buf: [u8; size_of::<Ipv4Addr>()] = [0; size_of::<Ipv4Addr>()];
        stream.read_exact(&mut ip_buf)?;

        Ok(Ipv4Addr::from(ip_buf))
    }

    fn recv_nodes(stream: &mut TcpStream) -> BoxResult<Vec<Node>> {
        let mut nodes = vec![];

        let mut id_buf: [u8; 1] = [0; 1];

        stream.read_exact(&mut id_buf)?;
        while id_buf != EOB {
            let ip = Directory::recv_ip(stream)?;
            let node = Node { id: id_buf[0], ip };
            nodes.push(node);

            stream.read_exact(&mut id_buf)?;
        }

        Ok(nodes)
    }

    fn parse_nodes(mut nodes: Vec<Node>) -> (Id2Ip, Ip2Id) {
        let mut id2ip = HashMap::new();
        let mut ip2id = HashMap::new();

        while let Some(node) = nodes.pop() {
            id2ip.insert(node.id, node.ip);
            ip2id.insert(node.ip, node.id);
        }

        (id2ip, ip2id)
    }
}
