//! System logging module.

use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};

use crate::common::{config::LoggerConfig, utils};

// ACTOR ----------------------------------------------------------------------

/// Logger is an entity <Actor> that keeps a reference to the file
/// that outputs the logging. It uses LogMessage to receive a log.

pub struct Logger {
    file: File,
}

impl Logger {
    /// Given a LoggerConfig this method will create a Logger entity, openning the associated file.

    pub fn new(LoggerConfig { dirpath }: LoggerConfig) -> Self {
        fs::create_dir_all(&dirpath).expect("[CRITICAL] Error while creating logs directory");
        let path = format!("{}/part2-{}.txt", &dirpath, utils::now_rfc());
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
        let c = format!("[{}] {}\n", utils::now_h_m_s(), msg.0);
        print!("{}", c);
        self.file
            .write_all(c.as_bytes())
            .expect("[CRITICAL] Write to file failed");
    }
}
