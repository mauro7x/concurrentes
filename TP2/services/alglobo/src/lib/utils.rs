use std::env;

use crate::{constants::env::FAILURE_RATE, types::common::BoxResult};

use log::*;
use rand::{thread_rng, Rng};

// ----------------------------------------------------------------------------

pub fn fail_randomly() -> BoxResult<()> {
    let coin = thread_rng().gen_range(0.0..1.0);
    match coin < failure_rate()? {
        true => {
            println!("<BOOM> (coin: {})", coin);
            Err("Random failure".into())
        }
        false => Ok(()),
    }
}

fn failure_rate() -> BoxResult<f64> {
    let failure_rate = env::var(FAILURE_RATE)
        .unwrap_or_else(|_| "0.01".to_string())
        .parse()?;

    Ok(failure_rate)
}
