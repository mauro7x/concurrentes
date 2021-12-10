use std::{env, error::Error};

pub struct Config {
    pub port: u32,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let port: u32 = env::var("PORT")?.parse()?;

        Ok(Config { port })
    }
}
