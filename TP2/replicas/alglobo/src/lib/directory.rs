use std::{
    collections::HashMap,
    fmt,
    io::{Read, Write},
    mem::size_of,
    net::{Ipv4Addr, SocketAddr, TcpStream},
    thread::sleep,
};

use crate::{
    constants::{DIRECTORY_CONNECTION_MAX_ATTEMPTS, DIRECTORY_CONNECTION_RETRY_TIME},
    types::{BoxResult, Id, Node},
};

// ----------------------------------------------------------------------------
// Protocol

type SendOpcode = [u8; 1];
const REGISTER: SendOpcode = [b'R'];
const FINISHED: SendOpcode = [b'F'];

type RecvOpcode = [u8; 1];
const ACCEPTED: RecvOpcode = [b'A'];
const REJECTED: RecvOpcode = [b'R'];
const EOB: RecvOpcode = [b'E'];

// ----------------------------------------------------------------------------

pub struct Directory {
    id: Id,
    addr: SocketAddr,
    stream: TcpStream,
    nodes: HashMap<Id, Ipv4Addr>,
}

impl Directory {
    pub fn new(addr: SocketAddr) -> BoxResult<Self> {
        let mut stream = Directory::connect_with_attemps(addr, DIRECTORY_CONNECTION_MAX_ATTEMPTS)?;
        let (id, nodes) = Directory::register(&mut stream)?;

        let ret = Directory {
            id,
            addr,
            stream,
            nodes,
        };
        Ok(ret)
    }

    pub fn finish(&self) -> BoxResult<()> {
        let mut stream = TcpStream::connect(self.addr)?;
        stream.write(&FINISHED)?;

        Ok(())
    }

    pub fn get_my_id(&self) -> Id {
        self.id
    }

    // Abstract

    fn connect_with_attemps(addr: SocketAddr, max_attemps: usize) -> BoxResult<TcpStream> {
        let mut attempts = 0;

        loop {
            match TcpStream::connect(addr) {
                Ok(stream) => return Ok(stream),
                Err(err) => {
                    if attempts == max_attemps {
                        println!("[ERROR] Max directory connection attemps, aborting");
                        Err(err)?;
                    };

                    attempts += 1;
                    sleep(DIRECTORY_CONNECTION_RETRY_TIME);
                }
            }
        }
    }

    fn register(stream: &mut TcpStream) -> BoxResult<(Id, HashMap<Id, Ipv4Addr>)> {
        // Start registration process by sending OPCODE
        stream.write(&REGISTER)?;

        // Receive fields according to protocol
        Directory::recv_accepted(stream)?;
        let id = Directory::recv_id(stream)?;
        let nodes = Directory::recv_nodes(stream)?;

        // Parse nodes
        let parsed_nodes = Directory::parse_nodes(nodes);

        Ok((id, parsed_nodes))
    }

    fn recv_accepted(stream: &mut TcpStream) -> BoxResult<()> {
        let mut buf: [u8; 1] = [0; 1];
        stream.read(&mut buf)?;

        match buf {
            ACCEPTED => {}
            REJECTED => {
                println!("[WARN] Register rejected because directory is full, aborting");
                Err("Rejected: Directory full")?;
            }
            [_] => {
                println!("[ERROR] Received unknown message from Directory, aborting");
                Err("Unknown message from Directory")?;
            }
        }

        Ok(())
    }

    fn recv_id(stream: &mut TcpStream) -> BoxResult<Id> {
        let mut id_buf: [u8; size_of::<Id>()] = [0; size_of::<Id>()];
        stream.read(&mut id_buf)?;

        Ok(id_buf[0])
    }

    fn recv_nodes(stream: &mut TcpStream) -> BoxResult<Vec<Node>> {
        let mut nodes = vec![];

        let mut id_buf: [u8; 1] = [0; 1];
        let mut addr_buf: [u8; 4] = [0; 4];

        stream.read(&mut id_buf)?;
        while id_buf != EOB {
            stream.read(&mut addr_buf)?;
            let node = Node {
                id: id_buf[0],
                addr: Ipv4Addr::from(addr_buf),
            };
            nodes.push(node);

            stream.read(&mut id_buf)?;
        }

        Ok(nodes)
    }

    fn parse_nodes(mut nodes: Vec<Node>) -> HashMap<Id, Ipv4Addr> {
        let mut parsed_nodes = HashMap::new();

        while let Some(node) = nodes.pop() {
            parsed_nodes.insert(node.id, node.addr);
        }

        parsed_nodes
    }
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.nodes.len() == 0 {
            return write!(f, "{:=^19}\n={:^17}=\n{:=^19}", "", "Empty directory", "");
        };

        write!(
            f,
            "{:=^19}\n={:^6}={:^10}=\n{:=^19}",
            "", "ID", "ADDRESS", ""
        )?;

        for node in &self.nodes {
            write!(f, "\n={:^4}={:^9}=", node.0, node.1)?;
        }

        write!(f, "\n{:=^16}", "")
    }
}
