use lib::constants;
use lib::protocol::{recv_msg, send_msg_to};
use lib::types::{Action, Entity, Message, Tx};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::env;
use std::net::UdpSocket;

// TODO: extract to config file (or env variable?)
const FAILURE_RATE: f64 = 0.3;
pub struct Service {
    name: Entity,
    conn: UdpSocket,
    failure_rate: f64,
    tx_log: HashMap<Tx, Action>,
}

impl Service {
    fn new(name: String, port: String) -> Self {
        let addr = format!("{}:{}", name, port);
        let conn = UdpSocket::bind(addr).expect("new: Failed to bind socket");

        let entity = match name.as_str() {
            "airline" => Entity::Airline,
            "bank" => Entity::Bank,
            "hotel" => Entity::Hotel,
            _ => panic!("Unknow entity name {}", name),
        };

        Service {
            name: entity,
            conn,
            failure_rate: FAILURE_RATE, // TODO: config
            tx_log: HashMap::new(),
        }
    }

    fn prepare_tx(&mut self, alglobo: &String, req: &Message) -> Result<(), std::io::Error> {
        let coin = thread_rng().gen_range(0.0..1.0);
        // random failure
        if coin < self.failure_rate {
            return self.abort_tx(alglobo, req);
        }
        // TODO: bank: reserve resources
        self.tx_log.insert(req.tx, Action::Prepare);
        println!("[tx {}] preparing...", req.tx);
        self.respond_message(alglobo, req, Action::Prepare)
    }

    fn commit_tx(&mut self, alglobo: &String, req: &Message) -> Result<(), std::io::Error> {
        // TODO: bank: remove resources
        self.tx_log.insert(req.tx, Action::Commit);
        println!("[tx {}] committing...", req.tx);
        self.respond_message(alglobo, req, Action::Commit)
    }

    fn abort_tx(&mut self, alglobo: &String, req: &Message) -> Result<(), std::io::Error> {
        // TODO: bank: restore resources
        self.tx_log.insert(req.tx, Action::Abort);
        println!("[tx {}] aborting...", req.tx);
        self.respond_message(alglobo, req, Action::Abort)
    }

    fn respond_message(
        &mut self,
        alglobo: &String,
        req: &Message,
        action: Action,
    ) -> Result<(), std::io::Error> {
        let res = Message {
            from: self.name,
            action,
            tx: req.tx,
        };

        match send_msg_to(&mut self.conn, &res, alglobo) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn process_txs(&mut self) -> Result<(), std::io::Error> {
        let (src, req) =
            recv_msg(&mut self.conn).expect("[ERROR] No se pudo recibir el mensaje por el socket.");
        println!("TENGO UN MENSAJE NUEVO!");
        match (self.tx_log.get(&req.tx), req.action) {
            // valid action flows
            (None, Action::Prepare) => self.prepare_tx(&src, &req),
            (Some(Action::Prepare), Action::Commit) => self.commit_tx(&src, &req),
            (Some(Action::Prepare), Action::Abort) => self.abort_tx(&src, &req),
            // action has been already processed
            (Some(logged_action), new_action) if (*logged_action == new_action) => {
                println!(
                    "[tx {}] resending already processed response for action {:?}",
                    req.tx, req.action
                );
                self.respond_message(&src, &req, req.action)
            }
            // retrying an aborted transaction
            (Some(Action::Abort), Action::Prepare) => {
                println!("[tx {}] retrying previously aborted transaction", req.tx);
                self.prepare_tx(&src, &req)
            }
            // retrying a previously committed transaction -> do nothing and resend status
            (Some(Action::Commit), Action::Prepare) => {
                println!("[tx {}] transaction has already been committed", req.tx);
                self.respond_message(&src, &req, Action::Commit)
            }
            // communication issue, we did not receive the prepare and transaction was aborted
            (None, Action::Abort) => {
                // abort transaction but do NOT release resources, since they were not reserved
                self.tx_log.insert(req.tx, Action::Abort);
                println!("[tx {}] aborting new transaction", req.tx);
                self.respond_message(&src, &req, Action::Abort)
            }
            // invalid action flow (should never happen)
            _ => panic!("process_txs: invalid action sequence detected"),
        }
    }

    fn run(&mut self) {
        loop {
            self.process_txs().expect("run: failed processing txs");
        }
    }
}

fn main() {
    println!("ESTOY VIVO!");
    let svc_name = env::var(constants::SVC_HOSTNAME).expect("SVC_HOSTNAME env variable undefined");
    let svc_port = env::var(constants::SVC_PORT).expect("SVC_PORT env variable undefined");

    let mut svc: Service = Service::new(svc_name, svc_port);
    svc.run()
}
