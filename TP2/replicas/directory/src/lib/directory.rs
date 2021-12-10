use std::{
    collections::HashSet,
    error::Error,
    io::{ErrorKind, Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
    vec,
};

use crate::{
    config::Config,
    node::Node,
    protocol::{
        encode, msg_from, RecvMessage, ACCEPTED, DEAD, EMPTY_MESSAGE, EOB, FINISHED, NEW, REGISTER,
        REJECTED,
    },
    types::*,
    utils::next,
};

// Constants --------------------------------------------------------

const POLLING_SLEEP_TIME: Duration = Duration::from_millis(100);

// Implementation -------------------------------------------------------------

pub struct Directory {
    max_nodes: Id,
    listener: TcpListener,
    finished: bool,
    next_id: Id,
    used_ids: HashSet<Id>,
    nodes: Vec<Node>,
}

impl Directory {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let Config { port, max_nodes } = Config::new()?;

        let nodes = Vec::with_capacity(max_nodes.into());
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        let ret = Directory {
            max_nodes,
            listener,
            finished: false,
            next_id: 0,
            used_ids: HashSet::new(),
            nodes,
        };

        Ok(ret)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!(
            "[INFO] Accepting connections on port {}...",
            self.listener.local_addr()?.port()
        );

        while !self.finished {
            let result = self.listener.accept();
            match result {
                Ok(connection) => self.connection_handler(connection)?,
                Err(err) => match err.kind() {
                    ErrorKind::WouldBlock => sleep(POLLING_SLEEP_TIME),
                    _ => panic!("[ERROR] {}", err),
                },
            }
        }

        println!("[INFO] Terminated gracefully. Bye bye!");

        Ok(())
    }

    fn get_next_id(&mut self) -> Id {
        while !self.used_ids.insert(self.next_id) {
            self.next_id = next(self.next_id);
        }

        let id = self.next_id;
        self.next_id = next(id);

        id
    }

    fn connection_handler(
        &mut self,
        connection: (TcpStream, SocketAddr),
    ) -> Result<(), Box<dyn Error>> {
        let (mut stream, addr) = connection;

        let ip = addr.ip();
        println!("[INFO] New connection from {}", ip);

        let mut buf: RecvMessage = EMPTY_MESSAGE;
        if let Err(err) = stream.read(&mut buf) {
            println!(
                "[WARN] Error ({}) while reading from {}, aborting connection",
                err, ip
            );
            return Ok(());
        }

        match buf {
            REGISTER => self.register(ip, stream)?,
            FINISHED => self.finished = true,

            // Unknown
            _ => {
                println!(
                    "[WARN] Unknown command received from {}, aborting connection",
                    ip
                );
            }
        };

        Ok(())
    }

    fn register(&mut self, ip: IpAddr, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        println!("[INFO] {} asked to be registered!", ip);

        if self.used_ids.len() == self.max_nodes.into() {
            if let Err(err) = stream.write(&[REJECTED]) {
                println!(
                    "[WARN] Error while responding REJECTED to {}: {} (ignoring)",
                    ip, err
                );
            }
            return Ok(());
        }

        let old_id = self.next_id;
        let id = self.get_next_id();
        let mut node = Node { id, ip, stream };
        if let Err(err) = self.broadcast_current_to(&mut node) {
            println!(
                "[WARN] Error while broadcasting current state to {}: {} (ignoring)",
                ip, err
            );
            self.next_id = old_id;
            return Ok(());
        };
        self.broadcast_new(node)?;
        println!("[INFO] {} joined the network with id: {}", ip, id);

        Ok(())
    }

    fn broadcast_current_to(&self, node: &mut Node) -> Result<(), Box<dyn Error>> {
        let mut stream = &node.stream;

        stream.write(&[ACCEPTED])?;
        for node in &self.nodes {
            let encoded_node = encode(node)?;
            stream.write(&encoded_node)?;
        }
        stream.write(&[EOB])?;

        Ok(())
    }

    fn broadcast_new(&mut self, new_node: Node) -> Result<(), Box<dyn Error>> {
        let mut nodes = vec![];
        let mut dead_nodes = vec![];

        let msg = msg_from(NEW, &new_node)?;
        for mut node in self.nodes.pop() {
            if node.ip == new_node.ip {
                continue;
            };

            match node.stream.write(&msg) {
                Ok(_) => nodes.push(node),
                Err(_) => {
                    self.used_ids.remove(&node.id);
                    dead_nodes.push(node);
                }
            };
        }
        nodes.push(new_node);
        self.nodes = nodes;

        if dead_nodes.len() > 0 {
            self.broadcast_dead(dead_nodes)?;
        };

        Ok(())
    }

    fn broadcast_dead(&mut self, dead_nodes: Vec<Node>) -> Result<(), Box<dyn Error>> {
        println!("[INFO] Removing dead nodes: {:#?}", dead_nodes);
        let mut nodes = vec![];
        let mut more_dead_nodes = vec![];

        let mut msgs = Vec::<Vec<u8>>::new();
        for dead_node in dead_nodes {
            let msg = msg_from(DEAD, &dead_node)?;
            msgs.push(msg);
        }

        for mut node in self.nodes.pop() {
            let mut error = false;
            for msg in &msgs {
                if let Err(_) = node.stream.write(msg) {
                    error = true;
                    break;
                };
            }

            match error {
                false => nodes.push(node),
                true => {
                    self.used_ids.remove(&node.id);
                    more_dead_nodes.push(node);
                }
            }
        }
        self.nodes = nodes;

        if more_dead_nodes.len() > 0 {
            self.broadcast_dead(more_dead_nodes)?;
        };

        Ok(())
    }
}

// ----------------------------------------------------------------------------
