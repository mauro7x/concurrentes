use std::collections::VecDeque;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};

use crate::part2::webservice::{Book, WebService};

pub struct WebServiceDispatcher {
    name: String,
    rate_limit: isize,
    pending_reqs: VecDeque<u64>,
    retry_time: u64,
    service: Addr<WebService>,
}

impl WebServiceDispatcher {
    pub fn new(
        service: Addr<WebService>,
        name: String,
        rate_limit: isize,
        retry_time: u64,
    ) -> Self {
        WebServiceDispatcher {
            name,
            pending_reqs: VecDeque::new(),
            rate_limit,
            retry_time,
            service,
        }
    }

    fn send_all_possible_requests(&mut self, ctx: &mut Context<Self>) {
        while self.rate_limit > 0 && !self.pending_reqs.is_empty() {
            let req_id;
            match self.pending_reqs.pop_front() {
                Some(id) => {
                    req_id = id;
                }
                None => {
                    panic!("[CRITICAL] Attempted to pop out of an empty queue")
                } // this case should never happen...
            }
            let next_booking = Book {
                req_id,
                requester: ctx.address(),
            };
            self.service.try_send(next_booking); // TODO: Handle result
            self.rate_limit -= 1;
        }
    }
}

impl Actor for WebServiceDispatcher {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[WebServiceDispatcher] {} started", self.name);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct HandleBook {
    pub req_id: u64,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BookSucceeded {
    pub req_id: u64,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BookFailed {
    pub req_id: u64,
}

impl Handler<HandleBook> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: HandleBook, ctx: &mut Context<Self>) {
        println!(
            "[{} WebServiceDispatcher] handle request id: {}",
            self.name, msg.req_id
        );

        self.pending_reqs.push_back(msg.req_id);
        self.send_all_possible_requests(ctx);
    }
}

impl Handler<BookSucceeded> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: BookSucceeded, ctx: &mut Context<Self>) {
        println!(
            "[{} WebServiceDispatcher] book succeeded msg received id: {}",
            self.name, msg.req_id
        );

        self.rate_limit += 1;
    }
}

impl Handler<BookFailed> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: BookFailed, ctx: &mut Context<Self>) {
        println!(
            "[{} WebServiceDispatcher] book failed msg received id: {}",
            self.name, msg.req_id
        );
        // TODO: Handle retry_time
        self.rate_limit += 1;
        ctx.address().try_send(HandleBook { req_id: msg.req_id });
    }
}
