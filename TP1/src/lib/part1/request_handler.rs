use std::thread::{self, JoinHandle};

use crate::common::utils::*;
use crate::part1::{
    airlines::{Airline, Airlines},
    hotel::Hotel,
    logger::LoggerSender,
    metrics_collector::MetricsSender,
    request::Request,
};

pub struct InvalidRequest;

pub struct RequestHandler {
    logger_sender: LoggerSender,
    metrics_sender: MetricsSender,
    threads: Vec<JoinHandle<()>>,
    next_id: u32,
    airlines: Airlines,
    hotel: Hotel,
}

fn handler(
    req_id: u32,
    airline: Airline,
    mut hotel: Option<Hotel>,
    req: Request,
    logger_sender: LoggerSender,
    metrics_sender: MetricsSender,
) {
    let ts_start = now();
    logger_sender.send(format!("[REQ #{}] -- START --", req_id));

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
    logger_sender.send(format!(
        "[REQ #{}] -- FINISHED -- (time: {} ms, retries: {})",
        req_id, duration_ms, retries
    ));
    metrics_sender.send(req, duration_ms);
}

impl RequestHandler {
    pub fn new(
        airlines: Airlines,
        hotel: Hotel,
        logger_sender: LoggerSender,
        metrics_sender: MetricsSender,
    ) -> Self {
        RequestHandler {
            logger_sender,
            metrics_sender,
            threads: Vec::new(),
            next_id: 0,
            airlines,
            hotel,
        }
    }

    pub fn handle(&mut self, req: Request) -> Result<(), InvalidRequest> {
        let airline = self.airlines.get(&req.airline).ok_or(InvalidRequest)?;

        let airline_cln = airline.clone();
        let hotel_cln = match req.package {
            true => Some(self.hotel.clone()),
            false => None,
        };

        let req_id = self.next_id;
        let logger_sender = self.logger_sender.clone();
        let metrics_sender = self.metrics_sender.clone();
        let join_handler = thread::spawn(move || {
            handler(
                req_id,
                airline_cln,
                hotel_cln,
                req,
                logger_sender,
                metrics_sender,
            )
        });
        self.threads.push(join_handler);
        self.next_id += 1;

        Ok(())
    }

    pub fn join(mut self) {
        for join_handler in self.threads {
            if let Err(err) = join_handler.join() {
                self.logger_sender.send(format!(
                    "[WARNING] Error while joining RequestHandler. Error: {:?}",
                    err
                ))
            }
        }

        self.threads = Vec::new();
    }
}
