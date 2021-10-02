use std::{collections::HashMap, error::Error, sync::Arc};

use crate::{
    config::{AirlineConfig, AirlinesConfig},
    webservice::WebService,
};

pub type Airline = Arc<WebService>;
pub type Airlines = HashMap<String, Airline>;

pub fn from_path(path: &str) -> Result<Airlines, Box<dyn Error>> {
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
            Arc::new(WebService::new(name, rate_limit, failure_rate, retry_time)),
        );
    }

    Ok(content)
}
