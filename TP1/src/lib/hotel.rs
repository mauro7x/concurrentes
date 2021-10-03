use std::{error::Error, sync::Arc};

use crate::{config::HotelConfig, webservice::WebService};

pub type Hotel = Arc<WebService>;

pub fn from_path(path: &str) -> Result<Hotel, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?;

    let HotelConfig {
        name,
        rate_limit,
        failure_rate,
        retry_time,
    } = serde_json::from_str(&data)?;

    Ok(Arc::new(WebService::new(
        name,
        rate_limit,
        failure_rate,
        retry_time,
    )))
}
