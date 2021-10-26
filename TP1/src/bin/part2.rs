use std::env;

use actix::Actor;
use actix_web::{web::Data, App, HttpServer};

use lib::common::{config::GeneralConfig, paths};
use lib::part2::{
    logger::Logger,
    metrics::MetricsCollector,
    request_handler::RequestHandler,
    routes::{get_index, get_metrics, get_request, post_request},
    state::ServerState,
    status_service::StatusService,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let GeneralConfig {
        port,
        logger_config,
        metrics_collector_config,
    } = GeneralConfig::from_path(paths::GENERAL_CONFIG)
        .expect("[CRITICAL] Error reading general config");

    let port = match env::var("PORT") {
        Ok(var) => var.parse::<u16>().expect("[CRITICAL] Invalid PORT env var"),
        Err(_) => port,
    };

    let logger = Logger::new(logger_config).start();
    let metrics_collector = MetricsCollector::new(metrics_collector_config, logger.clone()).start();
    let status_service = StatusService::new(logger.clone(), metrics_collector.clone()).start();
    let request_handler = RequestHandler::new(logger.clone(), status_service.clone()).start();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ServerState::new(
                request_handler.clone(),
                status_service.clone(),
                logger.clone(),
                metrics_collector.clone(),
            )))
            .service(get_index)
            .service(get_metrics)
            .service(post_request)
            .service(get_request)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
