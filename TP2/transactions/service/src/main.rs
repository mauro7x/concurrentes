use std::net::UdpSocket;
use std::env;
use lib::constants;
use lib::protocol::{Action, Entity, Message, recv_msg};

pub struct Service {
    name: String,
    conn: UdpSocket
}

impl Service {
    fn new(name: String, port: String) -> Self {
        let addr = format!("{}:{}", name, port);
        let conn = UdpSocket::bind(addr).expect("new: Failed to bind socket");

        Service {
            name,
            conn
        }
    }

    fn test_send_recv(&mut self) -> bool {
        let msg = match recv_msg(&mut self.conn) {
            Ok(msg) => msg,
            Err(err) => panic!("{:#?}", err)
        };

        true
    }
}

fn main() {
    let svc_name = env::var(constants::SVC_HOSTNAME).expect("SVC_HOSTNAME env variable undefined");
    let svc_port = env::var(constants::SVC_PORT).expect("SVC_PORT env variable undefined");

    let mut svc: Service = Service::new(svc_name, svc_port);
    svc.test_send_recv();
}
