use std::env;

use lib::constants;

fn main() {
    match env::var(constants::SVC_NAME) {
        Ok(name) => println!("Hello world from {}!", name),
        Err(err) => println!("Error: {:?}", err)
    };
}
