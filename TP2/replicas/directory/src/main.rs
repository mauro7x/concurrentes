use std::{error::Error, process::exit};

use lib::{constants::ERROR_CODE, directory::Directory};

// ----------------------------------------------------------------------------

fn run() -> Result<(), Box<dyn Error>> {
    let mut directory = Directory::new()?;
    directory.run()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("[ERROR] {}", err);
        exit(ERROR_CODE);
    }
}
