use lib::{service::Service, types::common::BoxResult};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// ----------------------------------------------------------------------------

fn run() -> BoxResult<()> {
    debug!("Creating...");
    let mut service = Service::new()?;

    debug!("Running...");
    service.run()?;

    debug!("Terminated gracefully");

    Ok(())
}

fn main() {
    pretty_env_logger::init();

    while let Err(err) = run() {
        error!("Crashed with error:\n{}", err);
        info!("Restarting...");
    }
}
