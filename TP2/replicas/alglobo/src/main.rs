use lib::{constants::RESTART_TIME, replica::Replica, types::BoxResult};
use std::thread::sleep;

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
