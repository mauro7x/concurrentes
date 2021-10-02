use std::sync::Arc;

use std_semaphore::Semaphore;

use crate::{fetch::*, utils::*};

pub struct WebService {
    name: String,
    sem: Arc<Semaphore>,
    failure_rate: f64,
    retry_time: u64,
}

impl WebService {
    pub fn new(name: String, rate_limit: isize, failure_rate: f64, retry_time: u64) -> Self {
        WebService {
            name,
            sem: Arc::new(Semaphore::new(rate_limit)),
            failure_rate,
            retry_time,
        }
    }

    pub fn fetch(&self, req_id: u32) -> Result<(), FetchError> {
        let _guard = self.sem.access();
        println!("[REQ #{}] Fetching {}...", req_id, self.name);
        simulate_fetch(self.failure_rate)
    }

    pub fn fetch_with_retries(&self, req_id: u32) -> u32 {
        let mut retries: u32 = 0;

        println!("[REQ #{}] Waiting to fetch {}...", req_id, self.name);
        loop {
            if let Ok(()) = self.fetch(req_id) {
                return retries;
            };
            println!(
                "[REQ #{}] Fetch to {} failed! Retrying in {} secs.",
                req_id, self.name, self.retry_time
            );
            sleep(self.retry_time);
            retries += 1;
            println!(
                "[REQ #{}] Waiting to fetch {}... (retries: {})",
                req_id, self.name, retries
            );
        }
    }
}
