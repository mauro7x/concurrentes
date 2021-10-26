use actix::Addr;

use crate::part2::{
    logger::Logger, metrics::MetricsCollector, request_handler::RequestHandler,
    status_service::StatusService,
};

pub struct ServerState {
    pub metrics_collector: Addr<MetricsCollector>,
    pub request_handler: Addr<RequestHandler>,
    pub status_service: Addr<StatusService>,
    pub logger: Addr<Logger>,
}

impl ServerState {
    pub fn new(
        request_handler: Addr<RequestHandler>,
        status_service: Addr<StatusService>,
        logger: Addr<Logger>,
        metrics_collector: Addr<MetricsCollector>,
    ) -> Self {
        ServerState {
            request_handler,
            status_service,
            logger,
            metrics_collector,
        }
    }
}
