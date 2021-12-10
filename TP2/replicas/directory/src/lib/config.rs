use crate::types::*;
use std::{env, error::Error};

pub struct Config {
    pub port: u32,
    pub max_nodes: Id,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let port = env::var("PORT")?.parse()?;

        let max_nodes = env::var("MAX_NODES")?.parse()?;
        assert!(max_nodes < Id::MAX);

        Ok(Config { port, max_nodes })
    }
}
