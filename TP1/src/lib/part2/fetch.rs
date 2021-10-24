use rand::Rng;

pub struct FetchError;

pub fn simulate_fetch(failure_rate: f64) -> Result<(), FetchError> {
    let mut rng = rand::thread_rng();

    // Simulate fetch
    // let fetch_time = rng.gen_range(1..20);
    // TODO: CAMBIAR POR COSA RARA DE BOX!!!!
    // sleep(fetch_time);

    // Simulate status
    let coin = rng.gen_range(0.0..1.0);
    match coin > failure_rate {
        true => Ok(()),
        false => Err(FetchError),
    }
}