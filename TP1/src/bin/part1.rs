use std::{error::Error, process};

use lib::common::{config::GeneralConfig, paths};

use lib::part1::{
    airlines, dispatcher, hotel, logger, metrics_collector, request_handler::RequestHandler,
};

fn run() -> Result<(), Box<dyn Error>> {
    let GeneralConfig {
        logger_config,
        metrics_collector_config,
    } = GeneralConfig::from_path(paths::GENERAL)?;
    let logger = logger::Logger::from_config(logger_config)?;
    let metrics_collector =
        metrics_collector::MetricsCollector::from_config(metrics_collector_config)?;
    let airlines = airlines::from_path(paths::AIRLINES_CONFIG, logger.get_sender())?;
    let hotel = hotel::from_path(paths::HOTEL_CONFIG, logger.get_sender())?;
    let mut req_handler = RequestHandler::new(
        airlines,
        hotel,
        logger.get_sender(),
        metrics_collector.get_sender(),
    );

    dispatcher::from_path(paths::REQUESTS, &mut req_handler)?;

    req_handler.join();
    logger.join();
    metrics_collector.join();
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
