use std::{thread, time};

use chrono::Utc;

pub fn now() -> i64 {
    let dt = Utc::now();
    dt.timestamp_millis()
}

pub fn sleep(secs: u64) {
    let duration = time::Duration::from_secs(secs);
    thread::sleep(duration);
}

pub fn clean_screen() {
    print!("{}[2J", 27 as char);
}
