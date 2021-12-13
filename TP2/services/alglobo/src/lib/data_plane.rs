use std::{
    collections::HashMap,
    fs::File,
    net::UdpSocket,
    sync::{Arc, MutexGuard},
    thread,
};

use crate::{
    config::data::Config,
    constants::{data::WAIT_ALL_RESPONSES_TIMEOUT, errors::MUTEX_LOCK_ERROR},
    protocol::data::recv_msg,
    service::{AirlineService, BankService, HotelService},
    types::{
        common::BoxResult,
        control::Shared,
        data::{Action, Entity, Message, Responses, Transaction, Tx},
    },
};

use csv::Reader;

// ----------------------------------------------------------------------------

pub struct DataPlane {
    payments_file: Reader<File>,
    responses: Arc<Shared<Responses>>,
    bank_service: BankService,
    hotel_service: HotelService,
    airline_service: AirlineService,
    socket: UdpSocket,
    tx_log: HashMap<Tx, Action>,
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
        DataPlane::populate_responses(&mut service_responses);

        let responses = Arc::new(Shared::new(service_responses));

        let ret = DataPlane {
            responses,
            bank_service: BankService::new(bank_addr),
            airline_service: AirlineService::new(airline_addr),
            hotel_service: HotelService::new(hotel_addr),
            socket,
            tx_log: HashMap::new(),
            payments_file: csv::Reader::from_path(payments_file_path)?,
        };

        let mut receiver = ret.receiver()?;
        thread::spawn(move || receiver.run());

        Ok(ret)
    }

    pub fn run_iteration(&mut self) -> BoxResult<()> {
        let mut iter = self.payments_file.deserialize();
        if let Some(result) = iter.next() {
            let record: Transaction = result?;
            self.process_tx(&record)?;
        }

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
        let res = Message {
            from: Entity::AlGlobo,
            action,
            tx,
        };

        if let Err(err) = self.inner_broadcast_message(res) {
            if action != Action::Abort {
                self.abort_tx(tx)?;
            }

            return Err(err);
        }

        Ok(())
    }

    fn inner_broadcast_message(&mut self, res: Message) -> BoxResult<()> {
        self.bank_service.send_message(&self.socket, &res)?;
        self.hotel_service.send_message(&self.socket, &res)?;
        self.airline_service.send_message(&self.socket, &res)?;

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
            "[tx {}] all responses received. Action: {:?} Responses: [{}] Response action: {:?}",
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

    fn populate_responses(service_responses: &mut HashMap<Entity, Option<Action>>) {
        service_responses.insert(Entity::Airline, None);
        service_responses.insert(Entity::Bank, None);
        service_responses.insert(Entity::Hotel, None);
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
