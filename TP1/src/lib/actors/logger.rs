extern crate actix;

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
};

use actix::{prelude::*, Actor, Context, Handler};

use crate::config::LoggerConfig;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LogMessage(pub String);

pub struct Logger {
    file: File,
}

impl Actor for Logger {
    type Context = Context<Self>;
}

// Simple message handler for Ping message
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

pub fn from_config(config: LoggerConfig) -> Result<Logger, Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(config.path)?;
    Ok(Logger { file })
}
