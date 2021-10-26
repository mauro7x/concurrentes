use std::collections::VecDeque;
use std::time::Duration;

use actix::{Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler, Message, ResponseActFuture, WrapFuture, clock::sleep};

use crate::part2::{
    logger::Logger,
    request::Request,
    status_service::{BookSucceeded, StatusService},
    webservice::{Book, WebService},
};

// TYPES ----------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum WebServiceType {
    Airline,
    Hotel,
}

// ACTOR ----------------------------------------------------------------------

pub struct WebServiceDispatcher {
    name: String,
    rate_limit: isize,
    pending_reqs: VecDeque<Request>,
    retry_time: u64,
    service: Addr<WebService>,
    logger: Addr<Logger>,
    status_service: Addr<StatusService>,
    webservice_type: WebServiceType,
}

impl WebServiceDispatcher {
    pub fn new(
        service: Addr<WebService>,
        name: String,
        rate_limit: isize,
        retry_time: u64,
        logger: Addr<Logger>,
        status_service: Addr<StatusService>,
        webservice_type: WebServiceType,
    ) -> Self {
        WebServiceDispatcher {
            name,
            pending_reqs: VecDeque::new(),
            rate_limit,
            retry_time,
            service,
            logger,
            status_service,
            webservice_type,
        }
    }

    fn book(&mut self, req: Request, addr: Addr<WebServiceDispatcher>) {
        Logger::send_to(
            &self.logger,
            format!("({}) Fetching for request {}", self.name, req.id),
        );
        self.service
            .try_send(Book {
                req,
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
        Logger::send_to(&self.logger, format!("({}) Dispatcher started", self.name));
    }
}

// MESSAGES -------------------------------------------------------------------

#[derive(Message)]
#[rtype(result = "()")]
pub struct HandleBook {
    pub req: Request,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FetchSucceeded {
    pub req: Request,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FetchFailed {
    pub req: Request,
}

// HANDLERS -------------------------------------------------------------------

impl Handler<HandleBook> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: HandleBook, ctx: &mut Context<Self>) {
        Logger::send_to(
            &self.logger,
            format!("({}) HandleBook for request {}", self.name, msg.req.id),
        );

        if self.rate_limit > 0 {
            self.book(msg.req, ctx.address());
            self.rate_limit -= 1;
        } else {
            Logger::send_to(
                &self.logger,
                format!("({}) Queueing request {}", self.name, msg.req.id),
            );
            self.pending_reqs.push_back(msg.req);
        }
    }
}

impl Handler<FetchSucceeded> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: FetchSucceeded, ctx: &mut Context<Self>) {
        Logger::send_to(
            &self.logger,
            format!("({}) FetchSucceeded for request {}", self.name, msg.req.id),
        );
        self.status_service
            .try_send(BookSucceeded {
                req: msg.req,
                book_type: self.webservice_type,
            })
            .expect("[CRITICAL] BookSucceeded sending failed");
        self.book_or_release(ctx.address());
    }
}

impl Handler<FetchFailed> for WebServiceDispatcher {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: FetchFailed, ctx: &mut Context<Self>) -> Self::Result {
        Logger::send_to(
            &self.logger,
            format!("({}) FetchFailed for request {}", self.name, msg.req.id),
        );
        self.book_or_release(ctx.address());

        // We wait retry_time until retrying the failed req
        Logger::send_to(
            &self.logger,
            format!(
                "({}) Waiting {} secs before retrying for request {}",
                self.name, self.retry_time, msg.req.id
            ),
        );
        Box::pin(
            sleep(Duration::from_secs(self.retry_time))
                .into_actor(self)
                .map(move |_result, _me, ctx| {
                    ctx.address()
                        .try_send(HandleBook { req: msg.req })
                        .expect("[CRITICAL] Could not send HandleBook msg to dispatcher");
                }),
        )
    }
}
