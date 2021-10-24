use std::collections::VecDeque;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};

use crate::part2::{
    request::Request,
    webservice::{Book, WebService},
};

pub struct WebServiceDispatcher {
    name: String,
    rate_limit: isize,
    pending_reqs: VecDeque<Request>,
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

    fn book(&mut self, req: Request, addr: Addr<WebServiceDispatcher>) {
        self.service
            .try_send(Book {
                req: req,
                requester: addr,
            })
            .expect("[CRITICAL] Error while fetching web service")
    }

    fn book_or_release(&mut self, addr: Addr<WebServiceDispatcher>) {
        match self.pending_reqs.pop_front() {
            Some(next_req) => self.book(next_req, addr),
            None => self.rate_limit += 1,
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
    pub req: Request,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BookSucceeded {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BookFailed {
    pub req: Request,
}

impl Handler<HandleBook> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: HandleBook, ctx: &mut Context<Self>) {
        println!(
            "[{} Dispatcher] HandleBook for request {}",
            self.name, msg.req.id
        );

        if self.rate_limit > 0 {
            self.book(msg.req, ctx.address());
            self.rate_limit -= 1;
        } else {
            self.pending_reqs.push_back(msg.req);
        }
    }
}

impl Handler<BookSucceeded> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, _: BookSucceeded, ctx: &mut Context<Self>) {
        println!("[{} Dispatcher] BookSucceeded received", self.name);
        self.book_or_release(ctx.address());
    }
}

impl Handler<BookFailed> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: BookFailed, ctx: &mut Context<Self>) {
        println!(
            "[{} Dispatcher] BookFailed for request {}",
            self.name, msg.req.id
        );

        self.book_or_release(ctx.address());

        // TODO: El encolamiento deberia pasar después de retry_time
        ctx.address()
            .try_send(HandleBook { req: msg.req })
            .expect("[CRITICAL] Could not send HandleBook msg to dispatcher");
    }
}
