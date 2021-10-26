//! Airline Webservice.
use std::{collections::HashMap, error::Error};

use crate::common::config::{AirlineConfig, AirlinesConfig};
use crate::part1::{logger::LoggerSender, webservice::WebService};

pub type Airline = WebService;

pub type Airlines = HashMap<String, Airline>;

/// Given a String representing a system path and a sender for the Logger
/// this method will create a map of airles that will handle each request correspondingly.
/// Each Airline is a WebService that controls the rate limit and simulates the fetch to the hotel provider.

pub fn from_path(path: &str, logger_sender: LoggerSender) -> Result<Airlines, Box<dyn Error>> {
    let mut content = Airlines::new();

    let data = std::fs::read_to_string(path)?;
    let airlines: AirlinesConfig = serde_json::from_str(&data)?;

    for AirlineConfig {
        name,
        rate_limit,
        failure_rate,
        retry_time,
        min_delay,
        max_delay,
    } in airlines
    {
        content.insert(
            name.clone(),
            WebService::new(
                name,
                rate_limit,
                failure_rate,
                retry_time,
                logger_sender.clone(),
                min_delay,
                max_delay,
            ),
        );
    }

    Ok(content)
}
