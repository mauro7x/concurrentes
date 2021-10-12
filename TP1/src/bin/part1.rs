use std::{error::Error, process};

use lib::{
    airlines, config::GeneralConfig, dispatcher, hotel, logger, paths,
    request_handler::RequestHandler,
};

fn parse_config(path: &str) -> Result<GeneralConfig, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?;
    let config: GeneralConfig = serde_json::from_str(&data)?;

    Ok(config)
}

fn run() -> Result<(), Box<dyn Error>> {
    let config: GeneralConfig = parse_config(paths::GENERAL)?;
    let logger = logger::Logger::from_config(&config.logger_config)?;
    let airlines = airlines::from_path(paths::AIRLINES_CONFIG, logger.get_sender())?;
    let hotel = hotel::from_path(paths::HOTEL_CONFIG, logger.get_sender())?;
    let mut req_handler = RequestHandler::new(airlines, hotel, logger.get_sender());

    dispatcher::from_path(paths::REQUESTS, &mut req_handler)?;

    req_handler.join();
    logger.join();
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
