use std::{collections::HashMap, error::Error};

use actix::{Actor, Addr};

use crate::common::{config::AirlineConfig, config::AirlinesConfig};
use crate::part2::{
    dispatcher::{WebServiceDispatcher, WebServiceType},
    logger::Logger,
    status_service::StatusService,
    webservice::WebService,
};

// TYPES ---------------------------------------------------------------------0

pub type Airline = Addr<WebServiceDispatcher>;
pub type Airlines = HashMap<String, Airline>;

// FUNCTIONS ------------------------------------------------------------------

pub fn from_path(
    path: &str,
    logger: Addr<Logger>,
    status_service: Addr<StatusService>,
) -> Result<Airlines, Box<dyn Error>> {
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
        let airline = WebService::new(
            name.clone(),
            failure_rate,
            min_delay,
            max_delay,
            logger.clone(),
        )
        .start();
        let dispatcher = WebServiceDispatcher::new(
            airline,
            name.clone(),
            rate_limit,
            retry_time,
            logger.clone(),
            status_service.clone(),
            WebServiceType::Airline,
        )
        .start();
        content.insert(name, dispatcher);
    }

    Ok(content)
}
