use std::env;

use lib::constants;

fn main() {
    // Get all env vars parsed in another file (some config)
    // to be used like:
    // let Config { name } = config_module::...
    match env::var(constants::SVC_NAME) {
        Ok(name) => println!("Hello world from {}!", name),
        Err(err) => println!("Error: {:?}", err)
    };
}
