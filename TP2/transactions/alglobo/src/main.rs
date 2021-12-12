use std::net::UdpSocket;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::{spawn};
use std::time::Duration;
use std::error::{Error};
use lib::constants;
use lib::file_logger::{FileLogger};
use lib::protocol::{recv_msg, send_msg_to};
use lib::types::{Action, Entity, Message, Status, Tx};

// TODO: extract to config file
const reqs_filename: &str = "./src/txs.txt";
const timeout: Duration = Duration::from_secs(30);
const failed_reqs_filename: &str = "./src/failed_reqs.txt";

pub struct AlGlobo {
    responses: Arc<(Mutex<HashMap<Entity, Option<Action>>>, Condvar)>,
    services_addr: HashMap<Entity, String>,
    socket: UdpSocket,
    tx_log: HashMap<Tx, Action>
}

fn populate_services_addr(services_addr: &mut HashMap<Entity, String>) {
    let airline_addr = env::var(constants::AIRLINE_ADDR).expect("AIRLINE_ADDR env variable undefined");
    let bank_addr = env::var(constants::BANK_ADDR).expect("BANK_ADDR env variable undefined");
    let hotel_addr = env::var(constants::HOTEL_ADDR).expect("HOTEL_ADDR env variable undefined");

    services_addr.insert(Entity::Airline, airline_addr);
    services_addr.insert(Entity::Bank, bank_addr);
    services_addr.insert(Entity::Hotel, hotel_addr);
}

fn populate_responses(service_responses: &mut HashMap<Entity, Option<Action>>) {
    service_responses.insert(Entity::Airline, None);
    service_responses.insert(Entity::Bank, None);
    service_responses.insert(Entity::Hotel, None);
}

impl AlGlobo {
    fn new(name: String, port: String) -> Self {
        let addr = format!("{}:{}", name, port);
        let socket = UdpSocket::bind(addr).expect("new: Failed to bind socket");

        let mut service_responses = HashMap::new();
        populate_responses(&mut service_responses);

        let responses = Arc::new((Mutex::new(service_responses), Condvar::new()));

        let mut services_addr = HashMap::new();
        populate_services_addr(&mut services_addr);

        let alglobo = AlGlobo {
            responses,
            services_addr,
            socket,
            tx_log: HashMap::new()
        };

        let mut alglobo_cln = alglobo.clone();
        spawn(move || alglobo_cln.process_responses());

        alglobo
    }

    fn clone(&self) -> Self {
        AlGlobo {
            responses: self.responses.clone(),
            services_addr: self.services_addr.clone(),
            socket: self.socket.try_clone().expect("clone: could not clone socket"),
            tx_log: self.tx_log.clone()
        }
    }

    fn reset_responses(&mut self) {
        let mut responses = self.responses.0
            .lock()
            .expect("reset_responses: could not acquire lock");
        for (_, action) in responses.iter_mut() {
            *action = None;
        }
    }

    fn broadcast_message(&mut self, tx: Tx, action: Action) -> Result<(), Box<dyn Error>> {
        let res = Message {
            from: Entity::AlGlobo,
            action,
            tx
        };

        for (_, addr) in &self.services_addr {
            match send_msg_to(&mut self.socket, &res, &addr) {
                Ok(_) => (),
                Err(err) => {
                    // there are unreachable services
                    // if we are not already aborting, we should abort tx
                    match action {
                        Action::Abort => return Err(Box::new(err)),
                        _ => {
                            self.abort_tx(tx)?;
                            return Err(Box::new(err));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn broadcast_message_and_wait(&mut self, tx: Tx, action: Action) -> Result<Action, Box<dyn Error>> {
        self.reset_responses();
        println!("[tx {}] broadcasting {:?}", tx, action);
        self.broadcast_message(tx, action)?;
        self.wait_all_responses(tx, action)
    }

    fn wait_all_responses(&mut self, tx: Tx, expected_action: Action) -> Result<Action, Box<dyn Error>> {
        let res = self.responses.1.wait_timeout_while(
            self.responses.0.lock().expect("wait_response: could not acquire guard lock"),
            timeout,
            |responses| responses.iter().any(|(_, v)| v.is_none())
        );

        match res {
            Err(err) => {
                panic!("wait_response: critical err: {}", err);
            },
            Ok((_, timeout_result)) if timeout_result.timed_out() => {
                // TODO: handle timeouts/retries?
                panic!("wait_response: timeout");
            }
            Ok((responses_guard, _)) => {
                self.process_result(&responses_guard, tx, expected_action)
            }
        }
    }

    fn process_result(&self, responses_guard: &MutexGuard<HashMap<Entity, Option<Action>>>, tx: Tx, expected_action: Action) -> Result<Action, Box<dyn Error>> {
        let mut response_action: Action = expected_action;
        let mut responses_str: String = String::from("");

        responses_guard
            .iter()
            .for_each(|(entity, res)| {
                responses_str.push_str(&format!("{:?}: {:?}, ", entity, res));
                match res {
                    Some(action) if *action == expected_action => (),
                    // service responded with abort
                    Some(action) if *action == Action::Abort => response_action = Action::Abort,
                    // service did not respond
                    None => response_action = Action::Abort,
                    // invalid case that should never occur
                    Some(action) =>
                        panic!("process_result: invalid response from server: action: {:?} expected_action {:?}", action, expected_action)
                };
            });

        println!("[tx {}] all responses received. Action: {:?} Responses: [{}] Response action: {:?}", tx, expected_action, responses_str, response_action);
        Ok(response_action)
    }

    fn process_txs_from_file(&mut self, file: &File, failed_requests_logger: &mut FileLogger) -> Result<(), Box<dyn Error>> {
        for line in BufReader::new(file).lines() {
            if let Ok(tx) = line?.parse::<u32>() {
                println!("starting to process tx: {}", tx);
                let action = self.process_tx(tx)?;
                failed_requests_logger.log(tx, &action);
            }
        }
        println!("Finished processing file transactions");
        Ok(())
    }

    fn process_txs(&mut self, file: &File, failed_requests_logger: &mut FileLogger) -> Result<(), Box<dyn Error>> {
        self.process_txs_from_file(file, failed_requests_logger)
    }

    fn process_tx(&mut self, tx: Tx) -> Result<Action, Box<dyn Error>> {
        // TODO: ESTADO COMPARTIDO
        match self.tx_log.get(&tx) {
            Some(Action::Commit) => self.commit_tx(tx),
            Some(Action::Abort) => self.abort_tx(tx),
            Some(Action::Prepare) | None => {
                match self.prepare_tx(tx)? {
                    Action::Prepare => self.commit_tx(tx),
                    Action::Abort => self.abort_tx(tx),
                    // commit should never be returned
                    Action::Commit => panic!("process_tx: prepare returned commit as response action")
                }
            },
        }
    }

    fn commit_tx(&mut self, tx: Tx) -> Result<Action, Box<dyn Error>> {
        self.tx_log.insert(tx, Action::Commit);
        self.broadcast_message_and_wait(tx, Action::Commit)
    }

    fn abort_tx(&mut self, tx: Tx) -> Result<Action, Box<dyn Error>> {
        self.tx_log.insert(tx, Action::Abort);
        self.broadcast_message_and_wait(tx, Action::Abort)
    }

    fn prepare_tx(&mut self, tx: Tx) -> Result<Action, Box<dyn Error>> {
        self.tx_log.insert(tx, Action::Prepare);
        self.broadcast_message_and_wait(tx, Action::Prepare)
    }

    fn process_response(&mut self, res: &Message) {
        // TODO: contemplate case of timeout and late response (transaction aborted)
        // Discard messages of other txs?
        self.responses.0.lock().expect("process_response: could not acquire lock").insert(res.from, Some(res.action));
        self.responses.1.notify_all();
    }

    fn process_responses(&mut self) {
        loop {
            let res = match recv_msg(&mut self.socket) {
                Ok((_, res)) => res,
                Err(err) => panic!("{:#?}", err)
            };
            self.process_response(&res);
        }
    }
}

fn main() {
    let svc_name = env::var(constants::SVC_HOSTNAME).expect("SVC_HOSTNAME env variable undefined");
    let svc_port = env::var(constants::SVC_PORT).expect("SVC_PORT env variable undefined");

    let mut failed_requests_logger = FileLogger::new(failed_reqs_filename);
    let file = File::open(reqs_filename).expect("Could not open txs file");

    let mut app: AlGlobo = AlGlobo::new(svc_name, svc_port);
    app.process_txs(&file, &mut failed_requests_logger);
}
