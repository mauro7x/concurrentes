use std::{
    error::Error,
    fmt::Debug,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, Shutdown, TcpStream},
    thread::sleep,
    time::Duration,
};

use lib::{
    config::Config,
    protocol::{ACCEPTED, EOB, REJECTED},
};

// ----------------------------------------------------------------------------

type Id = u8;

#[derive(Debug)]
struct Node {
    id: Id,
    addr: Ipv4Addr,
}

// ----------------------------------------------------------------------------

fn recv_register_response(directory: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; 1] = [0];
    directory.read(&mut buf)?;

    match &buf[0] {
        &ACCEPTED => {
            println!("[INFO] Registration accepted");
        }
        &REJECTED => {
            println!("[WARN] Register rejected because directory is full, aborting");
            Err("Rejected: Directory full")?;
        }
        _ => {
            println!("[ERROR] Received unknown message from Directory, aborting");
            Err("Unknown message from Directory")?;
        }
    };

    Ok(())
}

fn recv_id(directory: &mut TcpStream) -> Result<Node, Box<dyn Error>> {
    // Id (1)
    let mut id_buf: [u8; 1] = [0];
    directory.read(&mut id_buf)?;

    // Ipv4Addr (4)
    let mut addr_buf: [u8; 4] = [0, 0, 0, 0];
    directory.read(&mut addr_buf)?;

    Ok(Node {
        id: id_buf[0],
        addr: Ipv4Addr::from(addr_buf),
    })
}

fn recv_initial_nodes(directory: &mut TcpStream) -> Result<Vec<Node>, Box<dyn Error>> {
    let mut nodes = vec![];

    let mut id_buf: [u8; 1] = [0];
    let mut addr_buf: [u8; 4] = [0, 0, 0, 0];

    directory.read(&mut id_buf)?;
    while &id_buf[0] != &EOB {
        directory.read(&mut addr_buf)?;
        let node = Node {
            id: id_buf[0],
            addr: Ipv4Addr::from(addr_buf),
        };
        nodes.push(node);

        directory.read(&mut id_buf)?;
    }

    Ok(nodes)
}

fn optional_sleep() -> Result<(), Box<dyn Error>> {
    let sleep_time = std::env::var("SLEEP");
    match sleep_time {
        Ok(time) => {
            println!("[OPTIONAL SLEEP] Sleeping for {} secs", time);
            sleep(Duration::from_secs(time.parse()?));
            println!("[OPTIONAL SLEEP] Awaken");
        }
        Err(_) => {
            println!("[OPTIONAL SLEEP] No sleep");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from AlGlobo");
    let Config {
        port: _,
        directory_addr,
    } = Config::new()?;

    // Register
    sleep(Duration::from_secs(1)); // replace for some retrying system to connect
    let mut directory = TcpStream::connect(directory_addr)?;

    // Register request
    directory.write(&[b'R'])?;

    // Protocol
    // 1. Recv ACCEPTED/REJECTED (1)
    recv_register_response(&mut directory)?;

    // 2. Recv ID + ADDR (5)
    let myself = recv_id(&mut directory)?;
    println!("Received myself: {:?}", myself);

    // 3. Recv nodes ((ID + ADDR) * N) (5 * N)
    let nodes = recv_initial_nodes(&mut directory)?;
    println!("Received nodes: {:?}", nodes);

    optional_sleep()?;

    if let Err(err) = directory.shutdown(Shutdown::Both) {
        println!("Error while shutting down: {:?}", err)
    };

    Ok(())
}
