use std::io::{self, BufRead};

use crate::{config::data::Config, types::common::BoxResult};

// ----------------------------------------------------------------------------

pub fn run_manual_alglobo() -> BoxResult<()> {
    let config = Config::new()?;
    println!("Config: {:#?}", config);

    let stdin = io::stdin();
    println!("Reading user input..");
    for line in stdin.lock().lines() {
        println!("READ: {}", line.unwrap());
    }

    Ok(())
}
