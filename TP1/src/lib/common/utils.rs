use std::{thread, time};

use chrono::Utc;
use uuid::Uuid;

pub fn uuid() -> String {
    let my_uuid = Uuid::new_v4();
    my_uuid.to_string()
}

pub fn now() -> i64 {
    let dt = Utc::now();
    dt.timestamp_millis()
}

pub fn now_rfc() -> String {
    let dt = Utc::now();
    dt.to_rfc3339()
}

pub fn sleep(secs: u64) {
    let duration = time::Duration::from_secs(secs);
    thread::sleep(duration);
}

pub fn clean_screen() {
    print!("{}[2J", 27 as char);
}
