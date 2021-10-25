use std::time::Duration;

use actix::{
    clock::sleep, Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture,
    WrapFuture,
};
use rand::Rng;

use crate::part2::{
    dispatcher::{BookFailed, BookSucceeded, WebServiceDispatcher},
    request::Request,
};

// ACTOR ----------------------------------------------------------------------

pub struct WebService {
    pub name: String,
    failure_rate: f64,
    min_delay: u64,
    max_delay: u64,
}

impl WebService {
    pub fn new(name: String, failure_rate: f64, min_delay: u64, max_delay: u64) -> Self {
        WebService {
            name,
            failure_rate,
            min_delay,
            max_delay,
        }
    }
}

impl Actor for WebService {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[{}] WebService started", self.name);
    }
}

// MESSAGES -------------------------------------------------------------------

#[derive(Message)]
#[rtype(result = "()")]
pub struct Book {
    pub req: Request,
    pub requester: Addr<WebServiceDispatcher>,
}

// HANDLERS -------------------------------------------------------------------

impl Handler<Book> for WebService {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, Book { req, requester }: Book, _ctx: &mut Context<Self>) -> Self::Result {
        let mut rng = rand::thread_rng();
        let fetch_time = rng.gen_range(self.min_delay..self.max_delay);

        Box::pin(sleep(Duration::from_secs(fetch_time)).into_actor(self).map(
            move |_result, me, _ctx| {
                let coin = rng.gen_range(0.0..1.0);

                if coin > me.failure_rate {
                    requester
                        .try_send(BookSucceeded { req })
                        .expect("[CRITICAL] Could not send BookSucceeded msg");
                } else {
                    requester
                        .try_send(BookFailed { req })
                        .expect("[CRITICAL] Could not send BookFailed msg");
                }
            },
        ))
    }
}
