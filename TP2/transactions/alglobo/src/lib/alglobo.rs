use crate::{
    constants,
    file_logger::FileLogger,
    protocol::{recv_msg, send_msg_to},
    service::{AirlineService, BankService, HotelService},
    types::{Action, Entity, Message, Transaction, Tx},
};

use std::{
    collections::HashMap,
    env,
    error::Error,
    net::UdpSocket,
    sync::{Arc, Condvar, Mutex, MutexGuard},
    thread::spawn,
    time::Duration,
};

pub struct AlGlobo {
    responses: Arc<(Mutex<HashMap<Entity, Option<Action>>>, Condvar)>,
    bank_service: BankService,
    hotel_service: HotelService,
    airline_service: AirlineService,
    socket: UdpSocket,
    tx_log: HashMap<Tx, Action>,
}

const timeout: Duration = Duration::from_secs(10);

fn populate_services_addr(services_addr: &mut HashMap<Entity, String>) {
    let airline_addr =
        env::var(constants::AIRLINE_ADDR).expect("AIRLINE_ADDR env variable undefined");
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
    pub fn new(name: String, port: String) -> Self {
        let addr = format!("{}:{}", name, port);
        let socket = UdpSocket::bind(addr).expect("new: Failed to bind socket");

        let mut service_responses = HashMap::new();
        populate_responses(&mut service_responses);

        let responses = Arc::new((Mutex::new(service_responses), Condvar::new()));

        let mut services_addr = HashMap::new();
        populate_services_addr(&mut services_addr);

        let alglobo = AlGlobo {
            responses,
            bank_service: BankService::new(
                services_addr
                    .get(&Entity::Bank)
                    .expect("This shouldn't be possible!")
                    .clone(),
            ),
            airline_service: AirlineService::new(
                services_addr
                    .get(&Entity::Airline)
                    .expect("This shouldn't be possible!")
                    .clone(),
            ),
            hotel_service: HotelService::new(
                services_addr
                    .get(&Entity::Hotel)
                    .expect("This shouldn't be possible!")
                    .clone(),
            ),
            socket,
            tx_log: HashMap::new(),
        };

        let mut alglobo_cln = alglobo.clone();
        spawn(move || alglobo_cln.process_responses());

        alglobo
    }

    fn clone(&self) -> Self {
        AlGlobo {
            responses: self.responses.clone(),
            socket: self
                .socket
                .try_clone()
                .expect("clone: could not clone socket"),
            bank_service: self.bank_service.clone(),
            hotel_service: self.hotel_service.clone(),
            airline_service: self.airline_service.clone(),
            tx_log: self.tx_log.clone(),
        }
    }

    fn reset_responses(&mut self) {
        let mut responses = self
            .responses
            .0
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
            tx,
        };

        let mut do_steps = || -> std::io::Result<()> {
            self.bank_service.send_message(&mut self.socket, &res)?;
            self.hotel_service.send_message(&mut self.socket, &res)?;
            self.airline_service.send_message(&mut self.socket, &res)?;
            Ok(())
        };

        if let Err(err) = do_steps() {
            match action {
                Action::Abort => return Err(Box::new(err)),
                _ => {
                    self.abort_tx(tx)?;
                    return Err(Box::new(err));
                }
            }
        }

        Ok(())
    }

    fn broadcast_message_and_wait(
        &mut self,
        tx: Tx,
        action: Action,
    ) -> Result<Action, Box<dyn Error>> {
        self.reset_responses();
        println!("[tx {}] broadcasting {:?}", tx, action);
        self.broadcast_message(tx, action)?;
        self.wait_all_responses(tx, action)
    }

    fn wait_all_responses(
        &mut self,
        tx: Tx,
        expected_action: Action,
    ) -> Result<Action, Box<dyn Error>> {
        let res = self.responses.1.wait_timeout_while(
            self.responses
                .0
                .lock()
                .expect("wait_response: could not acquire guard lock"),
            timeout,
            |responses| responses.iter().any(|(_, v)| v.is_none()),
        );

        match res {
            Ok((_, timeout_result)) if timeout_result.timed_out() => {
                return Ok(Action::Abort);
            }
            Ok((responses_guard, _)) => {
                return self.process_result(&responses_guard, tx, expected_action);
            }
            Err(_) => {
                return Ok(Action::Abort);
            }
        };
    }

    fn process_result(
        &self,
        responses_guard: &MutexGuard<HashMap<Entity, Option<Action>>>,
        tx: Tx,
        expected_action: Action,
    ) -> Result<Action, Box<dyn Error>> {
        let mut response_action: Action = expected_action;
        let mut responses_str: String = String::from("");

        responses_guard.iter().for_each(|(entity, res)| {
            responses_str.push_str(&format!("{:?}: {:?}, ", entity, res));
            match res {
                Some(action) if *action == expected_action => (),
                // service responded with abort
                Some(action) if *action == Action::Abort => response_action = Action::Abort,
                // service did not respond
                None => response_action = Action::Abort,
                // invalid case that should never occur
                Some(action) => panic!(
          "process_result: invalid response from server: action: {:?} expected_action {:?}",
          action, expected_action
        ),
            };
        });

        println!(
            "[tx {}] all responses received. Action: {:?} Responses: [{}] Response action: {:?}",
            tx, expected_action, responses_str, response_action
        );
        Ok(response_action)
    }

    pub fn run(
        &mut self,
        payments_queue: &str,
        failed_requests_logger: &mut FileLogger,
    ) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(payments_queue)?;

        for result in rdr.deserialize() {
            let tx: Transaction = result?;
            let action = self.process_tx(&tx)?;
            // TODO HABILITAR MANEJO DE FALLAS
            failed_requests_logger.log(tx.id, &action);
        }
        Ok(())
    }

    fn process_tx(&mut self, tx: &Transaction) -> Result<Action, Box<dyn Error>> {
        // TODO: ESTADO COMPARTIDO
        match self.tx_log.get(&tx.id) {
            Some(Action::Commit) => self.commit_tx(tx.id),
            Some(Action::Abort) => self.abort_tx(tx.id),
            Some(Action::Prepare) | None => {
                match self.prepare_tx(tx.id)? {
                    Action::Prepare => self.commit_tx(tx.id),
                    Action::Abort => self.abort_tx(tx.id),
                    // commit should never be returned
                    Action::Commit => {
                        panic!("process_tx: prepare returned commit as response action")
                    }
                }
            }
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
        self.responses
            .0
            .lock()
            .expect("process_response: could not acquire lock")
            .insert(res.from, Some(res.action));
        self.responses.1.notify_all();
    }

    fn process_responses(&mut self) {
        loop {
            let res = match recv_msg(&mut self.socket) {
                Ok((_, res)) => res,
                Err(err) => panic!("{:#?}", err),
            };
            self.process_response(&res);
        }
    }
}
