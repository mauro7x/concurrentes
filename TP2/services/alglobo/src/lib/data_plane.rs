use csv::Reader;

use crate::types::BoxResult;

use crate::{
    config::data::Config,
    protocol::data::recv_msg,
    service::{AirlineService, BankService, HotelService},
    types::{Action, Entity, Message, Transaction, Tx},
};

use std::fs::File;
use std::{
    collections::HashMap,
    error::Error,
    net::UdpSocket,
    sync::{Arc, Condvar, Mutex, MutexGuard},
    thread::spawn,
    time::Duration,
};

// ----------------------------------------------------------------------------

pub struct DataPlane {
    payments_file: Reader<File>,
    responses: Arc<(Mutex<HashMap<Entity, Option<Action>>>, Condvar)>,
    bank_service: BankService,
    hotel_service: HotelService,
    airline_service: AirlineService,
    socket: UdpSocket,
    tx_log: HashMap<Tx, Action>,
}

struct DataPlaneReceiver {
    responses: Arc<(Mutex<HashMap<Entity, Option<Action>>>, Condvar)>,
    socket: UdpSocket,
}

impl DataPlane {
    pub fn new() -> BoxResult<Self> {
        let Config {
            port,
            hotel_addr,
            airline_addr,
            bank_addr,
            payments_file_path,
        } = Config::new()?;

        let addr = format!("0.0.0.0:{}", port);

        let socket = UdpSocket::bind(addr)?;

        let mut service_responses = HashMap::new();
        populate_responses(&mut service_responses);

        let responses = Arc::new((Mutex::new(service_responses), Condvar::new()));

        let alglobo = DataPlane {
            responses,
            bank_service: BankService::new(bank_addr),
            airline_service: AirlineService::new(airline_addr),
            hotel_service: HotelService::new(hotel_addr),
            socket,
            tx_log: HashMap::new(),
            payments_file: csv::Reader::from_path(payments_file_path)?,
        };

        let mut alglobo_cln = alglobo.receiver()?;
        spawn(move || alglobo_cln.process_responses());

        Ok(alglobo)
    }
    fn receiver(&self) -> BoxResult<DataPlaneReceiver> {
        Ok(DataPlaneReceiver {
            responses: self.responses.clone(),
            socket: self.socket.try_clone()?,
        })
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

    pub fn run_iteration(&mut self) -> Result<(), Box<dyn Error>> {
        let mut iter = self.payments_file.deserialize();

        if let Some(result) = iter.next() {
            let record: Transaction = result?;
            self.process_tx(&record)?;
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
}

impl DataPlaneReceiver {
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
const timeout: Duration = Duration::from_secs(10);

fn populate_responses(service_responses: &mut HashMap<Entity, Option<Action>>) {
    service_responses.insert(Entity::Airline, None);
    service_responses.insert(Entity::Bank, None);
    service_responses.insert(Entity::Hotel, None);
}
