use lib::{service::Service, types::common::BoxResult};

// ----------------------------------------------------------------------------

fn run() -> BoxResult<()> {
    let mut service = Service::new()?;
    service.run()?;

    Ok(())
}

fn main() {
    while let Err(err) = run() {
        println!("[ERROR] Crashed with error:");
        println!("{}", err);
        println!("[INFO] Restarting...");
    }
}
