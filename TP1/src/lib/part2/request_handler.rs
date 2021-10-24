use actix::{Actor, Context, Handler, Message};

use crate::common::{paths, utils};
use crate::part2::{
    airlines::{self, Airline, Airlines},
    dispatcher::HandleBook,
    errors::*,
    request::{RawRequest, Request},
};

// ACTOR ----------------------------------------------------------------------

pub struct RequestHandler {
    airlines: Airlines,
}

impl Actor for RequestHandler {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[RequestHandler] Started");
    }
}

impl RequestHandler {
    pub fn new() -> Self {
        let airlines = airlines::from_path(paths::AIRLINES_CONFIG)
            .expect("[CRITICAL] Error while initializing airlines");

        RequestHandler { airlines }
    }
}

// MESSAGES -------------------------------------------------------------------

#[derive(Message)]
#[rtype(result = "Result<String, HandlerError>")]
pub struct HandleRequest {
    pub raw_request: RawRequest,
}

// HANDLERS -------------------------------------------------------------------

impl Handler<HandleRequest> for RequestHandler {
    type Result = Result<String, HandlerError>;

    fn handle(&mut self, msg: HandleRequest, _: &mut Context<Self>) -> Self::Result {
        let raw_request = msg.raw_request;
        let req_id = utils::uuid();
        let req = Request {
            id: req_id.clone(),
            raw_request: raw_request.clone(),
        };

        println!("[REQ] {:#?}", req);

        let airline: &Airline = self
            .airlines
            .get(&raw_request.airline)
            .ok_or(HandlerError::AirlineNotFound)?;

        airline
            .try_send(HandleBook { req })
            .map_err(|_| HandlerError::AirlineUnavailable)?;

        Ok(req_id)
    }
}
