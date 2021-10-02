use std::{error::Error, process};

use lib::{airlines, dispatcher, hotel, paths, request_handler::RequestHandler};

fn run() -> Result<(), Box<dyn Error>> {
    let airlines = airlines::from_path(paths::AIRLINES_CONFIG)?;
    let hotel = hotel::from_path(paths::HOTEL_CONFIG)?;
    let mut req_handler = RequestHandler::new(airlines, hotel);

    dispatcher::from_path(paths::REQUESTS, &mut req_handler)?;

    req_handler.join();

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
