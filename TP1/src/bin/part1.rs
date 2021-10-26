use std::{env, error::Error, process};

use lib::common::{config::GeneralConfig, paths};

use lib::part1::{
    airlines, dispatcher, hotel, logger, metrics_collector, request_handler::RequestHandler,
};

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let GeneralConfig {
        port: _,
        logger_config,
        metrics_collector_config,
    } = GeneralConfig::from_path(paths::GENERAL_CONFIG)?;

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

    let path = match args.get(1) {
        Some(path) => {
            logger
                .get_sender()
                .send(format!("Using requests file received (path: {})", path));
            path
        }
        None => {
            logger.get_sender().send(format!(
                "No requests file received, using default one (path: {})",
                paths::DEFAULT_REQUESTS
            ));
            paths::DEFAULT_REQUESTS
        }
    };

    dispatcher::from_path(path, &mut req_handler)?;

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
