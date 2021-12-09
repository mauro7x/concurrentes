use std::net::UdpSocket;
use std::{thread, time}; // TMP
use std::collections::HashMap;
use std::env;
use lib::protocol::{Action, Entity, Message, recv_msg, send_msg_to};
use lib::{constants, protocol};

type Service = Services;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Services {
    Airline,
    Bank,
    Hotel
}

pub struct AlGlobo {
    name: String,
    services_addr: HashMap<Service, String>,
    socket: UdpSocket
}

fn populate_services_addr(services_addr: &mut HashMap<Service, String>) {
    let airline_addr = env::var(constants::AIRLINE_ADDR).expect("AIRLINE_ADDR env variable undefined");
    let bank_addr = env::var(constants::BANK_ADDR).expect("BANK_ADDR env variable undefined");
    let hotel_addr = env::var(constants::HOTEL_ADDR).expect("HOTEL_ADDR env variable undefined");

    services_addr.insert(Services::Airline, airline_addr);
    services_addr.insert(Services::Bank, bank_addr);
    services_addr.insert(Services::Hotel, hotel_addr);
}

impl AlGlobo {
    fn new(name: String, port: String) -> Self {
        let addr = format!("{}:{}", name, port);
        let socket = UdpSocket::bind(addr).expect("new: Failed to bind socket");

        let mut services_addr = HashMap::new();
        populate_services_addr(&mut services_addr);

        AlGlobo {
            name,
            services_addr,
            socket
        }
    }

    fn receive(&mut self) {
        loop {
            let msg = match recv_msg(&mut self.socket) {
                Ok(msg) => msg,
                Err(err) => panic!("{:#?}", err)
            };
        }
    }

    fn test_send_recv(&mut self) -> bool {
        println!("Sleeping for 5 secs");
        let five_secs = time::Duration::from_secs(5);
        thread::sleep(five_secs);

        let msg = Message {
            from: Entity::AlGlobo,
            action: Action::Prepare,
            tx: 1
        };

        for (service, addr) in &self.services_addr {
            send_msg_to(&mut self.socket, &msg, addr).expect("Failed to send msg");
        }

        self.receive();

        true
    }
}

fn main() {
    let svc_name = env::var(constants::SVC_HOSTNAME).expect("SVC_HOSTNAME env variable undefined");
    let svc_port = env::var(constants::SVC_PORT).expect("SVC_PORT env variable undefined");

    let mut app: AlGlobo = AlGlobo::new(svc_name, svc_port);
    app.test_send_recv();
}

// 1 - Conectar con algún servidor y mandar mensaje de ida y vuelta.
// 2 - Crear los mensajes y sus handlers.
// 3 - Recibir input e implementar lógica de mensajes
// 4 -