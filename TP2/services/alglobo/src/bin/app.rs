use std::{thread::sleep, time::Duration};

use lib::{replica::Replica, types::common::BoxResult};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// ----------------------------------------------------------------------------

pub const RESTART_TIME: Duration = Duration::from_secs(10);

// ----------------------------------------------------------------------------

fn run() -> BoxResult<()> {
    debug!("Creating...");
    let mut replica = Replica::new()?;

    debug!("Running...");
    replica.run()?;

    debug!("Terminated gracefully");

    Ok(())
}

fn hard_work_to_restart_our_complex_system() {
    sleep(RESTART_TIME);
}

fn main() {
    pretty_env_logger::init();

    while let Err(err) = run() {
        error!("Replica crashed with error: {}", err);

        info!("Restarting...");
        hard_work_to_restart_our_complex_system();
        info!("Restarted successfully");
    }
}
