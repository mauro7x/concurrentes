//! Hotel Webservice.
use std::error::Error;

use crate::common::config::HotelConfig;
use crate::part1::{logger::LoggerSender, webservice::WebService};

pub type Hotel = WebService;

/// Given a String representing a system path and a sender for the Logger
/// this method will create a hotel webservice that will handle each request correspondingly.
/// The Hotel is a WebService that controls the rate limit and simulates the fetch to the hotel provider.

pub fn from_path(path: &str, logger_sender: LoggerSender) -> Result<Hotel, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?;

    let HotelConfig {
        name,
        rate_limit,
        failure_rate,
        retry_time,
        min_delay,
        max_delay,
    } = serde_json::from_str(&data)?;

    Ok(WebService::new(
        name,
        rate_limit,
        failure_rate,
        retry_time,
        logger_sender,
        min_delay,
        max_delay,
    ))
}
