use std::{thread::sleep, time::Duration};

use lib::{replica::Replica, types::common::BoxResult};

// ----------------------------------------------------------------------------

pub const RESTART_TIME: Duration = Duration::from_secs(10);

// ----------------------------------------------------------------------------

fn run() -> BoxResult<()> {
    let mut replica = Replica::new()?;
    replica.run()?;

    Ok(())
}

fn hard_work_to_restart_our_complex_system() {
    sleep(RESTART_TIME);
}

fn main() {
    while let Err(err) = run() {
        println!("[ERROR] Replica crashed with error: {}", err);

        println!("[INFO] Restarting...");
        hard_work_to_restart_our_complex_system();
        println!("[INFO] Restarted successfully");
    }
}
