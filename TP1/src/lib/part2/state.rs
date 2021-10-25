use actix::Addr;

use crate::part2::{
    logger::Logger, request_handler::RequestHandler, status_service::StatusService,
};

pub struct ServerState {
    pub request_handler: Addr<RequestHandler>,
    pub status_service: Addr<StatusService>,
    pub logger: Addr<Logger>,
}

impl ServerState {
    pub fn new(
        request_handler: Addr<RequestHandler>,
        status_service: Addr<StatusService>,
        logger: Addr<Logger>,
    ) -> Self {
        ServerState {
            request_handler,
            status_service,
            logger,
        }
    }
}
