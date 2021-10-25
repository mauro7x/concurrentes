use std::error::Error;

use actix::{Actor, Addr};

use crate::common::config::HotelConfig;
use crate::part2::{
    dispatcher::{WebServiceDispatcher, WebServiceType},
    logger::Logger,
    status_service::StatusService,
    webservice::WebService,
};

// TYPES ---------------------------------------------------------------------0

pub type Hotel = Addr<WebServiceDispatcher>;

// FUNCTIONS ------------------------------------------------------------------

pub fn from_path(
    path: &str,
    logger: Addr<Logger>,
    status_service: Addr<StatusService>,
) -> Result<Addr<WebServiceDispatcher>, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?;

    let HotelConfig {
        name,
        rate_limit,
        failure_rate,
        retry_time,
        min_delay,
        max_delay,
    } = serde_json::from_str(&data)?;

    let hotel = WebService::new(
        name.clone(),
        failure_rate,
        min_delay,
        max_delay,
        logger.clone(),
    )
    .start();
    let dispatcher = WebServiceDispatcher::new(
        hotel,
        name,
        rate_limit,
        retry_time,
        logger,
        status_service,
        WebServiceType::Hotel,
    )
    .start();

    Ok(dispatcher)
}
