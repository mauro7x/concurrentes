use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};
use serde::Serialize;

use crate::common::utils::now;
use crate::part2::{
    dispatcher::WebServiceType, errors::StatusServiceError, logger::Logger,
    metrics::MetricsCollector, request::Request,
};

// TYPES ----------------------------------------------------------------------

pub struct RequestNotFound;

#[derive(Clone, Serialize)]
pub struct RequestStatus {
    pub req: Request,
    pub pending_hotel: bool,
    pub pending_airline: bool,
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
    metrics_collector: Addr<MetricsCollector>,
}

impl StatusService {
    pub fn new(logger: Addr<Logger>, metrics_collector: Addr<MetricsCollector>) -> Self {
        StatusService {
            reqs: HashMap::<String, RequestStatus>::new(),
            logger,
            metrics_collector,
        }
    }
}

impl Actor for StatusService {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        Logger::send_to(&self.logger, "[StatusService] Started".to_string());
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

    fn handle(&mut self, NewRequest { req }: NewRequest, _ctx: &mut Context<Self>) {
        let req_id = req.id.clone();
        self.reqs.insert(req_id.clone(), RequestStatus::new(req));
        Logger::send_to(
            &self.logger,
            format!("[StatusService] Registered request {}", req_id),
        );
    }
}

impl Handler<BookSucceeded> for StatusService {
    type Result = ();

    fn handle(
        &mut self,
        BookSucceeded { req, book_type }: BookSucceeded,
        _ctx: &mut Context<Self>,
    ) {
        let req_status = self
            .reqs
            .get_mut(&req.id)
            .expect("[CRITICAL] StatusService received BookSucceeded of unregistered request");

        match book_type {
            WebServiceType::Airline => {
                Logger::send_to(
                    &self.logger,
                    format!(
                        "[StatusService] Airline book registered for request {}",
                        req.id
                    ),
                );
                req_status.pending_airline = false;
            }
            WebServiceType::Hotel => {
                Logger::send_to(
                    &self.logger,
                    format!(
                        "[StatusService] Hotel book registered for request {}",
                        req.id
                    ),
                );
                req_status.pending_hotel = false;
            }
        }

        if !req_status.pending_hotel && !req_status.pending_airline {
            Logger::send_to(
                &self.logger,
                format!("[StatusService] Finished request {}", req.id),
            );
            MetricsCollector::collect(
                &self.metrics_collector,
                req.start_time,
                now(),
                req.raw_request.origin,
                req.raw_request.destiny,
            );
        }
    }
}

impl Handler<GetStatus> for StatusService {
    type Result = Result<RequestStatus, StatusServiceError>;

    fn handle(
        &mut self,
        GetStatus { req_id }: GetStatus,
        _ctx: &mut Context<Self>,
    ) -> Result<RequestStatus, StatusServiceError> {
        let req = self
            .reqs
            .get(&req_id)
            .ok_or(StatusServiceError::RequestNotFound)?;

        Logger::send_to(
            &self.logger,
            format!("[StatusService] Retrieved status for request {}", req_id),
        );

        Ok(req.clone())
    }
}
