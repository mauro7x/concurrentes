use std::{
    error::Error,
    io::Write,
    net::{Shutdown, TcpStream},
    thread::sleep,
    time::Duration,
};

use lib::config::Config;

// ----------------------------------------------------------------------------

fn optional_sleep() -> Result<(), Box<dyn Error>> {
    let sleep_time = std::env::var("SLEEP");
    match sleep_time {
        Ok(time) => {
            println!("[OPTIONAL SLEEP] Sleeping for {} secs", time);
            sleep(Duration::from_secs(time.parse()?));
            println!("[OPTIONAL SLEEP] Awaken");
        }
        Err(_) => {
            println!("[OPTIONAL SLEEP] No sleep");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from AlGlobo");
    let Config {
        port: _,
        directory_addr,
    } = Config::new()?;

    sleep(Duration::from_secs(1));

    // Register
    let mut directory = TcpStream::connect(directory_addr)?;

    directory.write(&[b'R'])?;

    optional_sleep()?;

    if let Err(err) = directory.shutdown(Shutdown::Both) {
        println!("Error while shutting down: {:?}", err)
    };

    Ok(())
}
