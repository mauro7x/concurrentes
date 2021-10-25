use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub logger_config: LoggerConfig,
    pub metrics_collector_config: MetricsCollectorConfig,
}

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct MetricsCollectorConfig {
    pub printer_period: u64,
    pub n_most_booked: usize,
}

#[derive(Debug, Deserialize)]
pub struct WebServiceConfig {
    pub name: String,
    pub rate_limit: isize,
    pub failure_rate: f64,
    pub retry_time: u64,
    pub min_delay: u64,
    pub max_delay: u64,
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
