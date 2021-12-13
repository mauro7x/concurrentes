use std::{
    collections::HashMap,
    fs::{self, File},
    net::UdpSocket,
    sync::{Arc, MutexGuard},
    thread,
};

use crate::{
    config::data::Config,
    constants::{
        data::WAIT_ALL_RESPONSES_TIMEOUT, errors::MUTEX_LOCK_ERROR, paths::PAYMENTS_TO_PROCESS,
        paths::TEMP_PAYMENTS_TO_PROCESS,
    },
    protocol::data::recv_msg,
    service_directory::ServiceDirectory,
    types::{
        common::BoxResult,
        control::Shared,
        data::{Action, Entity, Message, Responses, Transaction, Tx},
    },
};

use csv::Reader;

// ----------------------------------------------------------------------------

pub struct DataPlane {
    responses: Arc<Shared<Responses>>,
    socket: UdpSocket,
    tx_log: HashMap<Tx, Action>,
    services: ServiceDirectory,
}

impl DataPlane {
    pub fn new() -> BoxResult<Self> {
        println!("[DEBUG] (Data) Creating Config...");
        let Config {
            port,
            hotel_addr,
            airline_addr,
            bank_addr,
        } = Config::new()?;

        println!("[DEBUG] (Data) Creating service responses...");
        let responses = DataPlane::create_responses();

        println!("[DEBUG] (Data) Creating and binding socket...");
        let ret = DataPlane {
            responses: Arc::new(Shared::new(responses)),
            socket: UdpSocket::bind(format!("0.0.0.0:{}", port))?,
            tx_log: HashMap::new(),
            services: ServiceDirectory::new(airline_addr, bank_addr, hotel_addr),
        };

        println!("[DEBUG] (Data) Starting Receiver...");
        let mut receiver = ret.receiver()?;
        thread::spawn(move || receiver.run());

        Ok(ret)
    }

    pub fn run_iteration(&mut self) -> BoxResult<bool> {
        let mut payments_file = csv::Reader::from_path(PAYMENTS_TO_PROCESS)?;

        let mut iter = payments_file.deserialize();

        if let Some(result) = iter.next() {
            let record: Transaction = result?;
            self.process_tx(&record)?;
            self.update_payments_file(&mut payments_file)?;
            return Ok(true);
        }

        Ok(false)
    }
    fn update_payments_file(&mut self, payments_file: &mut Reader<File>) -> BoxResult<()> {
        let mut wtr = csv::Writer::from_path(TEMP_PAYMENTS_TO_PROCESS)?;

        wtr.write_record(payments_file.byte_headers()?)?;

        payments_file
            .byte_records()
            .try_for_each(|el| wtr.write_byte_record(&el?))?;

        wtr.flush()?;

        fs::copy(TEMP_PAYMENTS_TO_PROCESS, PAYMENTS_TO_PROCESS)?; // Copy foo.txt to bar.txt

        Ok(())
    }

    // Private

    fn receiver(&self) -> BoxResult<DataPlaneReceiver> {
        Ok(DataPlaneReceiver {
            responses: self.responses.clone(),
            socket: self.socket.try_clone()?,
        })
    }

    fn reset_responses(&mut self) -> BoxResult<()> {
        let mut responses = self.responses.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)?;
        for (_, action) in responses.iter_mut() {
            *action = None;
        }

        Ok(())
    }

    fn broadcast_message(&mut self, tx: Tx, action: Action) -> BoxResult<()> {
        let msg = Message {
            from: Entity::AlGlobo,
            action,
            tx,
        };

        if let Err(err) = self.services.broadcast(&self.socket, msg) {
            if action != Action::Abort {
                self.abort_tx(tx)?;
            }

            return Err(err);
        }

        Ok(())
    }

    fn broadcast_message_and_wait(&mut self, tx: Tx, action: Action) -> BoxResult<Action> {
        self.reset_responses()?;
        println!("[tx {}] broadcasting {:?}", tx, action);
        self.broadcast_message(tx, action)?;
        self.wait_all_responses(tx, action)
    }

    fn wait_all_responses(&mut self, tx: Tx, expected_action: Action) -> BoxResult<Action> {
        let res = self.responses.cv.wait_timeout_while(
            self.responses.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)?,
            WAIT_ALL_RESPONSES_TIMEOUT,
            |responses| responses.iter().any(|(_, v)| v.is_none()),
        );

        match res {
            Ok((_, timeout_result)) if timeout_result.timed_out() => Ok(Action::Abort),
            Ok((responses_guard, _)) => self.process_result(&responses_guard, tx, expected_action),
            Err(_) => Ok(Action::Abort),
        }
    }

    fn process_result(
        &self,
        responses_guard: &MutexGuard<HashMap<Entity, Option<Action>>>,
        tx: Tx,
        expected_action: Action,
    ) -> BoxResult<Action> {
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
            "[tx {}] All responses received. Action: {:?} Responses: [{}] Response action: {:?}",
            tx, expected_action, responses_str, response_action
        );
        Ok(response_action)
    }

    fn process_tx(&mut self, tx: &Transaction) -> BoxResult<Action> {
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

    fn commit_tx(&mut self, tx: Tx) -> BoxResult<Action> {
        self.tx_log.insert(tx, Action::Commit);
        self.broadcast_message_and_wait(tx, Action::Commit)
    }

    fn abort_tx(&mut self, tx: Tx) -> BoxResult<Action> {
        self.tx_log.insert(tx, Action::Abort);
        self.broadcast_message_and_wait(tx, Action::Abort)
    }

    fn prepare_tx(&mut self, tx: Tx) -> BoxResult<Action> {
        self.tx_log.insert(tx, Action::Prepare);
        self.broadcast_message_and_wait(tx, Action::Prepare)
    }

    // Abstract

    fn create_responses() -> Responses {
        let mut responses = HashMap::new();

        responses.insert(Entity::Airline, None);
        responses.insert(Entity::Bank, None);
        responses.insert(Entity::Hotel, None);

        responses
    }
}

// ----------------------------------------------------------------------------

struct DataPlaneReceiver {
    responses: Arc<Shared<Responses>>,
    socket: UdpSocket,
}

impl DataPlaneReceiver {
    fn run(&mut self) {
        if let Err(err) = self.process_responses() {
            // TODO: Avoid this panic, propagate!
            panic!("[ERROR] (Data) Crashed: {}", err)
        };
    }

    fn process_responses(&mut self) -> BoxResult<()> {
        loop {
            let res = match recv_msg(&self.socket) {
                Ok((_, res)) => res,
                Err(err) => panic!("{:#?}", err),
            };
            self.process_response(&res)?;
        }
    }

    fn process_response(&mut self, res: &Message) -> BoxResult<()> {
        // TODO: contemplate case of timeout and late response (transaction aborted)
        // Discard messages of other txs?
        self.responses
            .mutex
            .lock()
            .map_err(|_| MUTEX_LOCK_ERROR)?
            .insert(res.from, Some(res.action));
        self.responses.cv.notify_all();

        Ok(())
    }
}
