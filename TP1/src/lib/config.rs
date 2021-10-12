use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub logger_config: LoggerConfig,
}

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct WebServiceConfig {
    pub name: String,
    pub rate_limit: isize,
    pub failure_rate: f64,
    pub retry_time: u64,
}

pub type HotelConfig = WebServiceConfig;
pub type AirlineConfig = WebServiceConfig;
pub type AirlinesConfig = Vec<AirlineConfig>;

impl GeneralConfig {
    pub fn from_path(path: &str) -> Result<GeneralConfig, Box<dyn Error>> {
        let data = std::fs::read_to_string(path)?;
        let config: GeneralConfig = serde_json::from_str(&data)?;

        Ok(config)
    }
}
