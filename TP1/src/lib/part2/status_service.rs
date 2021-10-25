use crate::part2::{
    dispatcher::WebServiceType, errors::StatusServiceError, logger::Logger, request::Request,
};
use actix::{Actor, Addr, Context, Handler, Message};
use serde::Serialize;
use std::collections::HashMap;

// TYPES ----------------------------------------------------------------------

pub struct RequestNotFound;

#[derive(Clone, Serialize)]
pub struct RequestStatus {
    req: Request,
    pending_hotel: bool,
    pending_airline: bool,
}

impl RequestStatus {
    pub fn new(req: Request) -> Self {
        let package = req.raw_request.package;

        RequestStatus {
            req,
            pending_airline: true,
            pending_hotel: package,
        }
    }
}

// ACTOR ----------------------------------------------------------------------

pub struct StatusService {
    reqs: HashMap<String, RequestStatus>,
    logger: Addr<Logger>,
}

impl StatusService {
    pub fn new(logger: Addr<Logger>) -> Self {
        StatusService {
            reqs: HashMap::<String, RequestStatus>::new(),
            logger,
        }
    }
}

impl Actor for StatusService {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("StatusService started");
    }
}

// MESSAGES -------------------------------------------------------------------

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewRequest {
    pub req: Request,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BookSucceeded {
    pub book_type: WebServiceType,
    pub req: Request,
}

#[derive(Message)]
#[rtype(result = "Result<RequestStatus, StatusServiceError>")]
pub struct GetStatus {
    pub req_id: String,
}

// HANDLERS -------------------------------------------------------------------

impl Handler<NewRequest> for StatusService {
    type Result = ();

    fn handle(&mut self, msg: NewRequest, _ctx: &mut Context<Self>) {
        let req = msg.req;
        let req_id = req.id.clone();
        println!("[StatusService] Registering request {}", req_id);
        self.reqs.insert(req_id, RequestStatus::new(req));
    }
}

impl Handler<BookSucceeded> for StatusService {
    type Result = ();

    fn handle(&mut self, msg: BookSucceeded, _ctx: &mut Context<Self>) {
        let req_status = self
            .reqs
            .get_mut(&msg.req.id)
            .expect("[CRITICAL] StatusService received BookSucceeded of unregistered request");

        match msg.book_type {
            WebServiceType::AIRLINE => {
                println!(
                    "[StatusService] Registering airline book succeeded for request {}",
                    msg.req.id
                );
                req_status.pending_airline = false;
            }
            WebServiceType::HOTEL => {
                println!(
                    "[StatusService] Registering hotel book succeeded for request {}",
                    msg.req.id
                );
                req_status.pending_hotel = false;
            }
        }

        if !req_status.pending_hotel && !req_status.pending_airline {
            println!("[StatusService] FINISHED Request {}", msg.req.id);
        }
    }
}

impl Handler<GetStatus> for StatusService {
    type Result = Result<RequestStatus, StatusServiceError>;

    fn handle(
        &mut self,
        msg: GetStatus,
        _ctx: &mut Context<Self>,
    ) -> Result<RequestStatus, StatusServiceError> {
        println!("[StatusService] Getting status for request {}", msg.req_id);

        let req = self
            .reqs
            .get(&msg.req_id)
            .ok_or(StatusServiceError::RequestNotFound)?;

        Ok(req.clone())
    }
}
