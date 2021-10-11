use std::{error::Error, process};

use lib::{airlines, dispatcher, hotel, logger, paths, request_handler::RequestHandler};

fn run() -> Result<(), Box<dyn Error>> {
    let logger = logger::Logger::from_path(paths::GENERAL)?;
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
