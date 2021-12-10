use lib::directory::Directory;
use std::{error::Error, process::exit};

fn run() -> Result<(), Box<dyn Error>> {
    let mut directory = Directory::new()?;
    directory.run()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("[ERROR] {}", err);
        exit(-1);
    }
}
