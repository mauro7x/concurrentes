use std::{collections::HashMap, error::Error};

use actix::{Actor, Addr};

use crate::part2::{
    config::AirlineConfig, config::AirlinesConfig, dispatcher::WebServiceDispatcher,
    webservice::WebService,
};

pub type Airline = Addr<WebServiceDispatcher>;
pub type Airlines = HashMap<String, Airline>;

pub fn from_path(path: &str) -> Result<Airlines, Box<dyn Error>> {
    let mut content = Airlines::new();

    let data = std::fs::read_to_string(path)?;
    let airlines: AirlinesConfig = serde_json::from_str(&data)?;

    for AirlineConfig {
        name,
        rate_limit,
        failure_rate,
    } in airlines
    {
        let airline = WebService::new(name.clone(), failure_rate).start();
        let dispatcher = WebServiceDispatcher::new(airline, name.clone(), rate_limit).start();
        content.insert(name, dispatcher);
    }

    Ok(content)
}
