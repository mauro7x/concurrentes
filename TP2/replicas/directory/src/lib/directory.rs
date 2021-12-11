use std::{
    collections::HashSet,
    error::Error,
    io::{ErrorKind, Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    thread::sleep,
    vec,
};

use crate::{
    config::Config,
    constants::POLLING_SLEEP_TIME,
    node::Node,
    protocol::{
        encode, msg_from, RecvMessage, ACCEPTED, DEAD, EMPTY_MESSAGE, EOB, FINISHED, NEW, PING,
        REGISTER, REJECTED,
    },
    types::*,
    utils::next,
};

// ----------------------------------------------------------------------------

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

        println!("[INFO] Terminated gracefully");

        Ok(())
    }

    fn connection_handler(
        &mut self,
        connection: (TcpStream, SocketAddr),
    ) -> Result<(), Box<dyn Error>> {
        let (mut stream, addr) = connection;
        let ip = addr.ip();

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
        println!("[INFO] Register request from {}", ip);

        if self.full() && !self.remove_dead_nodes()? {
            println!(
                "[INFO] {} connection rejected since max_nodes being used",
                ip
            );

            if let Err(err) = stream.write(&[REJECTED]) {
                println!(
                    "[WARN] Error while responding REJECTED to {}: {} (ignoring)",
                    ip, err
                );
            }
            return Ok(());
        }

        let id = self.get_next_id();
        let mut node = Node { id, ip, stream };
        if let Err(err) = self.broadcast_current_to(&mut node) {
            println!(
                "[WARN] Error while broadcasting current state to {}: {} (ignoring)",
                ip, err
            );
            return Ok(());
        };
        self.broadcast_new(node)?;

        self.used_ids.insert(id);
        self.next_id = next(id);
        println!("[INFO] {} joined the network with id: {}", ip, id);
        self.print();

        Ok(())
    }

    fn broadcast_current_to(&self, node: &mut Node) -> Result<(), Box<dyn Error>> {
        let mut stream = &node.stream;

        // 1. Send accepted (1)
        stream.write(&[ACCEPTED])?;

        // 2. Send ID
        stream.write(&[node.id])?;

        // 3. Send rest of the nodes ((1 + 4 = 5) * N)
        for peer in &self.nodes {
            stream.write(&encode(peer)?)?;
        }

        // 4. Send EOB (1)
        stream.write(&[EOB])?;

        Ok(())
    }

    fn broadcast_new(&mut self, new_node: Node) -> Result<(), Box<dyn Error>> {
        let mut nodes = vec![];
        let mut dead_nodes = vec![];

        let msg = msg_from(NEW, &new_node)?;

        while let Some(mut node) = self.nodes.pop() {
            if node.ip == new_node.ip {
                self.used_ids.remove(&node.id);
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
        println!("[DEBUG] Removing detected dead nodes: {:#?}", dead_nodes);
        let mut nodes = vec![];
        let mut more_dead_nodes = vec![];

        let mut msgs = Vec::<Vec<u8>>::new();
        for dead_node in dead_nodes {
            let msg = msg_from(DEAD, &dead_node)?;
            msgs.push(msg);
        }

        while let Some(mut node) = self.nodes.pop() {
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

    // helpers

    fn get_next_id(&mut self) -> Id {
        let mut id = self.next_id;
        while self.used_ids.contains(&id) {
            id = next(id);
        }

        id
    }

    fn remove_dead_nodes(&mut self) -> Result<bool, Box<dyn Error>> {
        let mut nodes = vec![];
        let mut dead_nodes = vec![];

        while let Some(mut node) = self.nodes.pop() {
            match node.stream.write(&[PING]) {
                Ok(_) => nodes.push(node),
                Err(_) => {
                    self.used_ids.remove(&node.id);
                    dead_nodes.push(node);
                }
            };
        }
        self.nodes = nodes;

        let found_dead_nodes = dead_nodes.len() > 0;
        if found_dead_nodes {
            self.broadcast_dead(dead_nodes)?;
        };

        Ok(found_dead_nodes)
    }

    fn full(&self) -> bool {
        self.used_ids.len() == self.max_nodes.into()
    }

    fn print(&self) {
        if self.nodes.len() == 0 {
            return println!("{:=^27}\n={:^25}=\n{:=^27}", "", "Empty directory", "");
        };

        print!(
            "{:=^27}\n={:^6}|{:^18}=\n={:-^25}=",
            "", "ID", "ADDRESS", ""
        );
        for node in &self.nodes {
            print!("\n={:^6}|{:^18}=", node.id, node.ip);
        }
        println!("\n{:=^27}", "")
    }
}
