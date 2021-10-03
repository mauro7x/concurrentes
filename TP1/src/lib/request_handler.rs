use std::thread::{self, JoinHandle};

use crate::{
    airlines::{Airline, Airlines},
    hotel::Hotel,
    request::Request,
    utils::*,
};

pub struct InvalidRequest;

pub struct RequestHandler {
    threads: Vec<JoinHandle<()>>,
    next_id: u32,
    airlines: Airlines,
    hotel: Hotel,
}

fn handler(req_id: u32, airline: Airline, mut hotel: Option<Hotel>) {
    let ts_start = now();
    println!("[REQ #{}] -- START --", req_id);

    let hotel_thread: Option<JoinHandle<u32>> = hotel
        .take()
        .map(|hotel| thread::spawn(move || hotel.fetch_with_retries(req_id)));

    let retries = airline.fetch_with_retries(req_id);

    if let Some(join_handler) = hotel_thread {
        join_handler
            .join()
            .expect("[CRITICAL] Error while joining hotel thread.");
    }

    let ts_stop = now();
    let duration_ms = ts_stop - ts_start;
    println!(
        "[REQ #{}] -- FINISHED -- (time: {} ms, retries: {})",
        req_id, duration_ms, retries
    );
}

impl RequestHandler {
    pub fn new(airlines: Airlines, hotel: Hotel) -> Self {
        RequestHandler {
            threads: Vec::new(),
            next_id: 0,
            airlines,
            hotel,
        }
    }

    pub fn handle(&mut self, req: &Request) -> Result<(), InvalidRequest> {
        let airline = self.airlines.get(&req.airline).ok_or(InvalidRequest)?;

        let airline_cln = airline.clone();
        let hotel_cln = match req.package {
            true => Some(self.hotel.clone()),
            false => None,
        };

        let req_id = self.next_id;
        let join_handler = thread::spawn(move || handler(req_id, airline_cln, hotel_cln));
        self.threads.push(join_handler);
        self.next_id += 1;

        Ok(())
    }

    pub fn join(mut self) {
        for join_handler in self.threads {
            if let Err(err) = join_handler.join() {
                println!(
                    "[WARNING] Error while joining RequestHandler. Error: {:?}",
                    err
                )
            }
        }

        self.threads = Vec::new();
    }
}
