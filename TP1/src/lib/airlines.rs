use std::{collections::HashMap, error::Error};

use crate::{
    config::{AirlineConfig, AirlinesConfig},
    logger::LoggerSender,
    webservice::WebService,
};

pub type Airline = WebService;
pub type Airlines = HashMap<String, Airline>;

pub fn from_path(path: &str, logger_sender: LoggerSender) -> Result<Airlines, Box<dyn Error>> {
    let mut content = Airlines::new();

    let data = std::fs::read_to_string(path)?;
    let airlines: AirlinesConfig = serde_json::from_str(&data)?;

    for AirlineConfig {
        name,
        rate_limit,
        failure_rate,
        retry_time,
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
            ),
        );
    }

    Ok(content)
}
