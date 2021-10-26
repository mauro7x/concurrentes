use actix::{Actor, Addr, Context, Handler, Message};

use crate::common::{paths, utils, utils::now};
use crate::part2::{
    airlines::{self, Airline, Airlines},
    dispatcher::HandleBook,
    errors::*,
    hotel::{self, Hotel},
    logger::Logger,
    request::{RawRequest, Request},
    status_service::{NewRequest, StatusService},
};

// ACTOR ----------------------------------------------------------------------

pub struct RequestHandler {
    airlines: Airlines,
    hotel: Hotel,
    logger: Addr<Logger>,
    status_service: Addr<StatusService>,
}

impl Actor for RequestHandler {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        Logger::send_to(&self.logger, "[RequestHandler] Started".to_string());
    }
}

impl RequestHandler {
    pub fn new(logger: Addr<Logger>, status_service: Addr<StatusService>) -> Self {
        let airlines = airlines::from_path(
            paths::AIRLINES_CONFIG,
            logger.clone(),
            status_service.clone(),
        )
        .expect("[CRITICAL] Error while initializing airlines web services");
        let hotel = hotel::from_path(paths::HOTEL_CONFIG, logger.clone(), status_service.clone())
            .expect("[CRITICAL] Error while initializing hotel web service");

        RequestHandler {
            airlines,
            hotel,
            logger,
            status_service,
        }
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
            start_time: now(),
            raw_request: raw_request.clone(),
        };

        // In a real system, we should run the following
        // lines in transaction: that means, take some action
        // if message is correctly sent to the airline but not
        // to the hotel, for example.
        // Maybe we should do some rollback.

        self.status_service
            .try_send(NewRequest { req: req.clone() })
            .map_err(|_| HandlerError::StatusServiceUnavailable)?;

        let airline: &Airline = self
            .airlines
            .get(&raw_request.airline)
            .ok_or(HandlerError::AirlineNotFound)?;

        if raw_request.package {
            self.hotel
                .try_send(HandleBook { req: req.clone() })
                .map_err(|_| HandlerError::HotelUnavailable)?;
        }

        airline
            .try_send(HandleBook { req: req.clone() })
            .map_err(|_| HandlerError::AirlineUnavailable)?;

        Logger::send_to(&self.logger, format!("[RequestHandler] {:#?}", req));

        Ok(req_id)
    }
}
