use rand::Rng;

use crate::common::utils::*;

pub struct FetchError;

pub fn simulate_fetch(failure_rate: f64, min_delay: u64, max_delay: u64) -> Result<(), FetchError> {
    let mut rng = rand::thread_rng();

    // Simulate fetch
    let fetch_time = rng.gen_range(min_delay..max_delay);
    sleep(fetch_time);

    // Simulate status
    let coin = rng.gen_range(0.0..1.0);
    match coin > failure_rate {
        true => Ok(()),
        false => Err(FetchError),
    }
}
