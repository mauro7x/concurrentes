use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::ErrorKind,
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc, Mutex, MutexGuard,
    },
    thread,
};

use crate::{
    config::data::Config,
    constants::{
        data::{N_PREPARE_RETRIES, WAIT_ALL_RESPONSES_TIMEOUT},
        errors::MUTEX_LOCK_ERROR,
        general::NONBLOCKING_POLLING_RATE,
        paths::FAILED_PAYMENTS,
        paths::PAYMENTS_TO_PROCESS,
        paths::TEMP_PAYMENTS_TO_PROCESS,
    },
    protocol::data::unpack_message,
    service_directory::ServiceDirectory,
    thread_utils::{check_threads, safe_spawn},
    tx_log::TxLog,
    types::{
        common::BoxResult,
        control::{SafeThread, Shared},
        data::{Action, Entity, Message, Responses, Transaction, Tx},
    },
    utils::fail_randomly,
};

use csv::{ByteRecord, Reader, Writer};
use log::*;

// ----------------------------------------------------------------------------

pub struct DataPlane {
    current_tx: Arc<Mutex<Option<Tx>>>,
    responses: Arc<Shared<Responses>>,
    socket: UdpSocket,
    tx_log: TxLog,
    services: ServiceDirectory,
    threads: Vec<SafeThread>,
    stopped: Arc<AtomicBool>,
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
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))?;
        socket.set_nonblocking(true)?;

        let current_tx = Arc::new(Mutex::new(None));

        let mut ret = DataPlane {
            current_tx,
            responses: Arc::new(Shared::new(responses)),
            socket,
            tx_log: TxLog::new()?,
            services: ServiceDirectory::new(airline_addr, bank_addr, hotel_addr),
            threads: Vec::new(),
            stopped: Arc::new(AtomicBool::new(false)),
        };

        println!("[DEBUG] (Data) Starting Receiver...");
        let receiver = ret.receiver()?;
        safe_spawn(
            receiver,
            DataPlaneReceiver::process_responses,
            &mut ret.threads,
        )?;

        Ok(ret)
    }

    fn set_current_tx(&mut self, tx: Option<Tx>) -> BoxResult<()> {
        let mut current_tx = self.current_tx.lock().map_err(|_| MUTEX_LOCK_ERROR)?;
        *current_tx = tx;
        Ok(())
    }

    pub fn process_transaction(&mut self) -> BoxResult<bool> {
        let mut payments_file = csv::Reader::from_path(PAYMENTS_TO_PROCESS)?;

        let mut iter = payments_file.byte_records();

        if let Some(result) = iter.next() {
            let byte_record = result?;

            let tx: Transaction = (&byte_record).deserialize(None)?;
            self.set_current_tx(Some(tx.id))?;
            check_threads(&mut self.threads)?;

            match self.tx_log.get(&tx.id)? {
                Some(Action::Commit) => self.commit_tx(&tx)?,
                Some(Action::Abort) => self.abort_tx(&tx)?,
                Some(Action::Prepare) | None => {
                    match self.prepare_tx(&tx)? {
                        Action::Prepare => self.commit_tx(&tx)?,
                        Action::Abort => {
                            self.update_ret_file(&byte_record)?;
                            self.abort_tx(&tx)?
                        }
                        // commit should never be returned
                        Action::Commit => {
                            return Err(
                                "[ERROR] process_tx: prepare returned commit as response action"
                                    .into(),
                            );
                        }
                    };
                }
            };
            self.update_payments_file(&mut payments_file)?;
            self.set_current_tx(None)?;

            return Ok(true);
        }

        Ok(false)
    }

    // Private
    fn update_payments_file(&mut self, payments_file: &mut Reader<File>) -> BoxResult<()> {
        let mut wtr = Writer::from_path(TEMP_PAYMENTS_TO_PROCESS)?;

        wtr.write_record(payments_file.byte_headers()?)?;

        payments_file
            .byte_records()
            .try_for_each(|el| wtr.write_byte_record(&el?))?;

        wtr.flush()?;

        fs::copy(TEMP_PAYMENTS_TO_PROCESS, PAYMENTS_TO_PROCESS)?;

        Ok(())
    }

    fn update_ret_file(&mut self, byte_record: &ByteRecord) -> BoxResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(FAILED_PAYMENTS)?;

        let mut reader = Reader::from_reader(&file);

        let value: Transaction = byte_record.deserialize(None)?;

        for record in reader.deserialize() {
            let tx_to_retry: Transaction = record?;
            if tx_to_retry.id == value.id {
                return Ok(());
            }
        }

        let mut wtr = Writer::from_writer(&file);

        wtr.write_byte_record(byte_record)?;

        Ok(())
    }

    fn receiver(&self) -> BoxResult<DataPlaneReceiver> {
        Ok(DataPlaneReceiver {
            current_tx: self.current_tx.clone(),
            responses: self.responses.clone(),
            socket: self.socket.try_clone()?,
            stopped: self.stopped.clone(),
        })
    }

    fn reset_responses(&mut self) -> BoxResult<()> {
        let mut responses = self.responses.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)?;
        for (_, action) in responses.iter_mut() {
            *action = None;
        }

        Ok(())
    }

    fn broadcast_message(&mut self, tx: &Transaction, action: Action) -> BoxResult<()> {
        let msg = Message {
            from: Entity::AlGlobo,
            action,
            tx: *tx, // copy
        };

        self.services.broadcast(&self.socket, msg)
    }

    fn broadcast_until_getting_response_from_all_services(
        &mut self,
        tx: &Transaction,
        action: Action,
        n_retries: Option<u32>,
    ) -> BoxResult<Action> {
        let mut n_attempts = 0;
        let mut response: Option<Action> = None;

        while response.is_none() && (n_retries.is_none() || n_attempts < n_retries.unwrap()) {
            n_attempts += 1;
            println!(
                "[tx {}] broadcasting {:?} - {} attempt",
                tx.id, action, n_attempts
            );
            response = self.broadcast_message_and_wait(tx, action)?;
        }

        match response {
            Some(action) => Ok(action),
            None => Ok(Action::Abort),
        }
    }

    fn broadcast_message_and_wait(
        &mut self,
        tx: &Transaction,
        action: Action,
    ) -> BoxResult<Option<Action>> {
        check_threads(&mut self.threads)?;
        fail_randomly()?;
        self.reset_responses()?;
        self.broadcast_message(tx, action)?;
        self.wait_all_responses(tx.id, action)
    }

    fn wait_all_responses(&mut self, tx: Tx, expected_action: Action) -> BoxResult<Option<Action>> {
        let res = self.responses.cv.wait_timeout_while(
            self.responses.mutex.lock().map_err(|_| MUTEX_LOCK_ERROR)?,
            WAIT_ALL_RESPONSES_TIMEOUT,
            |responses| responses.iter().any(|(_, v)| v.is_none()),
        );

        match res {
            Ok((_, timeout_result)) if timeout_result.timed_out() => Ok(None),
            Ok((responses_guard, _)) => Ok(Some(self.process_result(
                &responses_guard,
                tx,
                expected_action,
            )?)),
            Err(_) => Ok(None),
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
                // should never happen that a service did not respond, since we wait
                // in the condition variable for all responses not being None
                None => panic!("process_result: there is a None response that should not be"),
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

    fn commit_tx(&mut self, tx: &Transaction) -> BoxResult<()> {
        self.tx_log.insert(tx.id, Action::Commit)?;
        self.broadcast_until_getting_response_from_all_services(tx, Action::Commit, None)?;
        Ok(())
    }

    fn abort_tx(&mut self, tx: &Transaction) -> BoxResult<()> {
        self.tx_log.insert(tx.id, Action::Abort)?;
        self.broadcast_until_getting_response_from_all_services(tx, Action::Abort, None)?;
        Ok(())
    }

    fn prepare_tx(&mut self, tx: &Transaction) -> BoxResult<Action> {
        self.tx_log.insert(tx.id, Action::Prepare)?;
        self.broadcast_until_getting_response_from_all_services(
            tx,
            Action::Prepare,
            Some(N_PREPARE_RETRIES),
        )
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

impl Drop for DataPlane {
    fn drop(&mut self) {
        println!("[DEBUG] (Data) Destroying...");
        self.stopped.store(true, Relaxed);
        while let Some(thread) = self.threads.pop() {
            thread.joiner.join().expect("Error joining threads");
        }
        println!("[DEBUG] (Data) Destroyed successfully");
    }
}

// ----------------------------------------------------------------------------

struct DataPlaneReceiver {
    current_tx: Arc<Mutex<Option<Tx>>>,
    responses: Arc<Shared<Responses>>,
    socket: UdpSocket,
    stopped: Arc<AtomicBool>,
}

impl DataPlaneReceiver {
    fn process_responses(&mut self) -> BoxResult<()> {
        while !self.stopped.load(Relaxed) {
            fail_randomly()?;
            self.recv_msg()?;
        }

        Ok(())
    }

    fn recv_msg(&mut self) -> BoxResult<()> {
        let mut buf = vec![0; 18];

        match self.socket.recv_from(&mut buf) {
            Ok(_) => {
                let msg = unpack_message(&buf)?;
                self.process_response(&msg)
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {
                    thread::sleep(NONBLOCKING_POLLING_RATE);
                    Ok(())
                }
                _ => Err(err.into()),
            },
        }
    }

    fn process_response(&mut self, res: &Message) -> BoxResult<()> {
        let current_tx: Option<Tx>;
        {
            current_tx = *self.current_tx.lock().map_err(|_| MUTEX_LOCK_ERROR)?;
        }
        match current_tx {
            Some(tx) if tx == res.tx.id => {
                self.responses
                    .mutex
                    .lock()
                    .map_err(|_| MUTEX_LOCK_ERROR)?
                    .insert(res.from, Some(res.action));
                self.responses.cv.notify_all();
                Ok(())
            }
            _ => {
                println!(
                    "[WARN] process_response: ignoring response. current_tx: {:?} != recved_tx {}",
                    current_tx, res.tx.id
                );
                Ok(())
            }
        }
    }
}
