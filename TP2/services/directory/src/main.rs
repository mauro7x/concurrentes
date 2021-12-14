use std::process::exit;

use lib::{constants::ERROR_CODE, directory::Directory, types::BoxResult};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// ----------------------------------------------------------------------------

fn run() -> BoxResult<()> {
    debug!("Creating...");
    let mut directory = Directory::new()?;

    debug!("Running...");
    directory.run()?;

    debug!("Terminated gracefully");

    Ok(())
}

fn main() {
    pretty_env_logger::init();

    if let Err(err) = run() {
        error!("{}", err);
        exit(ERROR_CODE);
    }
}
