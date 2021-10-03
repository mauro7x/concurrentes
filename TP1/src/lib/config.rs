use serde::Deserialize;

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
