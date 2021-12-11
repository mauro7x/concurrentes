use std::process::exit;

use lib::{constants::ERROR_CODE, directory::Directory, types::BoxResult};

// ----------------------------------------------------------------------------

fn run() -> BoxResult<()> {
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
