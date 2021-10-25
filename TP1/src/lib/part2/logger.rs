use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use actix::{Actor, Addr, Context, Handler, Message};

use crate::common::config::LoggerConfig;

// ACTOR ----------------------------------------------------------------------

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new(config: LoggerConfig) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(config.path)
            .expect("[CRITICAL] Error while opening logger file");

        Logger { file }
    }

    pub fn send_to(logger: Addr<Logger>, msg: String) {
        if let Err(_) = logger.try_send(LogMessage(msg)) {
            println!("Warning: failed to send log message to Logger");
        };
    }
}

impl Actor for Logger {
    type Context = Context<Self>;
}

// MESSAGES -------------------------------------------------------------------

#[derive(Message)]
#[rtype(result = "()")]
pub struct LogMessage(pub String);

// HANDLERS -------------------------------------------------------------------

impl Handler<LogMessage> for Logger {
    type Result = ();

    fn handle(&mut self, msg: LogMessage, _ctx: &mut Context<Self>) {
        let c = msg.0 + "\n";
        print!("{}", c);
        self.file
            .write_all(c.as_bytes())
            .expect("[CRITICAL] Write to file failed");
    }
}
