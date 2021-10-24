extern crate actix;
use actix::{prelude::*, Actor, Context, Handler};

pub struct Logger;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LogMessage(pub String);

impl Actor for Logger {
    type Context = Context<Self>;
}

// Simple message handler for Ping message
impl Handler<LogMessage> for Logger {
    type Result = ();

    fn handle(&mut self, msg: LogMessage, _ctx: &mut Context<Self>) {
        println!("{}", msg.0);
    }
}
