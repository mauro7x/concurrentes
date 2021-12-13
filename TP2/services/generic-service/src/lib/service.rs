use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    thread, time,
};

use rand::{thread_rng, Rng};

use crate::{
    config::Config,
    protocol::{recv_msg, send_msg_to},
    types::{
        common::BoxResult,
        data::{Action, Entity, Message, Tx},
    },
};

// ----------------------------------------------------------------------------

pub struct Service {
    name: Entity,
    socket: UdpSocket,
    failure_rate: f64,
    response_time_ms: u64,
    tx_log: HashMap<Tx, Action>,
}

impl Service {
    pub fn new() -> BoxResult<Self> {
        println!("[DEBUG] Creating Config...");
        let Config {
            name,
            port,
            failure_rate,
            response_time_ms,
        } = Config::new()?;

        println!("[DEBUG] Crating entity...");
        let entity = match name.as_str() {
            "airline" => Entity::Airline,
            "bank" => Entity::Bank,
            "hotel" => Entity::Hotel,
            _ => return Err(format!("Unknown entity name ({})", name).into()),
        };

        println!("[DEBUG] Creating and binding socket...");
        let ret = Self {
            name: entity,
            socket: UdpSocket::bind(format!("0.0.0.0:{}", port))?,
            failure_rate,
            response_time_ms,
            tx_log: HashMap::new(),
        };

        println!("[DEBUG] Service created successfully");

        Ok(ret)
    }

    pub fn run(&mut self) -> BoxResult<()> {
        loop {
            self.process_tx()?;
        }
    }

    pub fn process_tx(&mut self) -> BoxResult<()> {
        let (from, req) = recv_msg(&self.socket)?;
        let logged_action = self.tx_log.get(&req.tx);

        match (logged_action, req.action) {
            // Valid action flows
            (None, Action::Prepare) => self.prepare_tx(&from, &req),
            (Some(Action::Prepare), Action::Commit) => self.commit_tx(&from, &req),
            (Some(Action::Prepare), Action::Abort) => self.abort_tx(&from, &req),

            // Action has been already processed
            (Some(logged_action), req_action) if (*logged_action == req_action) => {
                println!(
                    "[tx {}] Resending already processed response for action {:?}",
                    req.tx, req.action
                );
                self.respond_message(&from, req.tx, req.action)
            }

            // Retrying an aborted transaction
            (Some(Action::Abort), Action::Prepare) => {
                println!("[tx {}] Retrying previously aborted transaction", req.tx);
                self.prepare_tx(&from, &req)
            }

            // Retrying a previously committed transaction -> do nothing and resend status
            (Some(Action::Commit), Action::Prepare) => {
                println!("[tx {}] Transaction has already been committed", req.tx);
                self.respond_message(&from, req.tx, Action::Commit)
            }

            // Communication issue
            // (we did not receive the prepare and transaction was aborted)
            (None, Action::Abort) => {
                // Abort transaction but do NOT release resources,
                // since they were not reserved
                self.tx_log.insert(req.tx, Action::Abort);
                println!("[tx {}] Aborting new transaction", req.tx);
                self.respond_message(&from, req.tx, Action::Abort)
            }

            // Invalid action flow (should never happen)
            (logged_action, req_action) => {
                return Err(format!(
                    "Invalid action sequence detected (logged_action: {:#?}, req_action: {:#?})",
                    logged_action, req_action
                )
                .into())
            }
        }
    }

    // Private

    fn log_and_respond(
        &mut self,
        action: Action,
        addr: &SocketAddr,
        req: &Message,
    ) -> BoxResult<()> {
        println!("[tx {}] Inserting <{:?}> action in log...", req.tx, action);
        self.tx_log.insert(req.tx, action);

        println!("[tx {}] Responding with action <{:?}>...", req.tx, action);
        self.respond_message(addr, req.tx, action)?;

        Ok(())
    }

    fn respond_message(&mut self, addr: &SocketAddr, tx: Tx, action: Action) -> BoxResult<()> {
        let msg = Message {
            from: self.name,
            action,
            tx,
        };

        let response_time = time::Duration::from_millis(self.response_time_ms);

        thread::sleep(response_time);

        send_msg_to(&self.socket, &msg, addr)?;

        Ok(())
    }

    fn should_reject_new_tx(&self) -> bool {
        let coin = thread_rng().gen_range(0.0..1.0);

        coin < self.failure_rate
    }

    // Handlers

    fn prepare_tx(&mut self, addr: &SocketAddr, req: &Message) -> BoxResult<()> {
        if self.should_reject_new_tx() {
            return self.abort_tx(addr, req);
        }

        // TODO: If Bank, reserve resources
        self.log_and_respond(Action::Prepare, addr, req)?;

        Ok(())
    }

    fn commit_tx(&mut self, addr: &SocketAddr, req: &Message) -> BoxResult<()> {
        // TODO: If Bank, remove resources
        self.log_and_respond(Action::Commit, addr, req)?;

        Ok(())
    }

    fn abort_tx(&mut self, addr: &SocketAddr, req: &Message) -> BoxResult<()> {
        // TODO: If Bank, restore resources
        self.log_and_respond(Action::Abort, addr, req)?;

        Ok(())
    }
}