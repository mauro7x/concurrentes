use std::{
    error::Error,
    io::{Read, Write},
    net::{Shutdown, TcpStream},
    time::Duration,
};

use lib::config::Config;

const TIMEOUT: Duration = Duration::from_secs(5);

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from AlGlobo");
    let Config { port } = Config::new()?;

    // Create socket
    // let addr = format!("0.0.0.0:{}", port);
    // let socket = UdpSocket::bind(addr).expect("Error while binding to addr");
    // let mut buf: [u8; 2] = [0, 0];

    // Register
    let directory = format!("localhost:{}", port);
    let mut dir_socket = TcpStream::connect(directory)?;

    let sleep_time = std::env::var("SLEEP");
    match sleep_time {
        Ok(time) => {
            println!("Sleeping for {} secs", time);
            std::thread::sleep(std::time::Duration::from_secs(time.parse()?));
            println!("Awaken. Leaving, bye bye!");
        }
        Err(_) => {
            println!("No sleep. Leaving, bye bye!");
        }
    }

    dir_socket.write(&[b'F'])?;

    // let mut buf: [u8; 1] = [0];
    // if let Err(err) = dir_socket.read(&mut buf) {
    //     println!("Error while reading from socket: {:?}", err);
    // } else {
    //     println!("Read: {:?}", buf);
    // }

    if let Err(err) = dir_socket.shutdown(Shutdown::Both) {
        println!("Error while shutting down: {:?}", err)
    };

    Ok(())
}
