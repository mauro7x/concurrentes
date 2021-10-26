//! System logging actor.

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};

use crate::common::{config::LoggerConfig, utils};

// ACTOR ----------------------------------------------------------------------

/// Logger is an entity <Actor> that keeps a reference to the file
/// that outputs the logging.

pub struct Logger {
    file: File,
}

impl Logger {
    /// Given a LoggerConfig this method will create a Logger entity, openning the associated file.

    pub fn new(config: LoggerConfig) -> Self {
        let path = format!("{}/part2-{}.txt", config.dirpath, utils::now_rfc());
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("[CRITICAL] Error while opening logger file");

        Logger { file }
    }

    /// Creates a LoggerMessage and sends it to the logger passed as argument.

    pub fn send_to(logger: &Addr<Logger>, msg: String) {
        if logger.try_send(LogMessage(msg)).is_err() {
            println!("Warning: failed to send log message to Logger");
        };
    }
}

impl Actor for Logger {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        Logger::send_to(&ctx.address(), "[Logger] Started".to_string());
    }
}

// MESSAGES -------------------------------------------------------------------

/// Message to log.
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
