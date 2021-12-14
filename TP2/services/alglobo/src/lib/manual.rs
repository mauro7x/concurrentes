use std::io::{self};

use crate::{types::{common::BoxResult}, data_plane::DataPlane};

// ----------------------------------------------------------------------------

fn prompt() {
    let expected_format = "tx_id,cbu,airline_cost,hotel_cost";
    println!("\nPlease enter the transaction to retry: (format: {})\n", expected_format);
}

pub fn run_manual_alglobo() -> BoxResult<()> {

    let mut data_plane = DataPlane::new(true)?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    prompt();
    for input in rdr.deserialize() {
        match input {
            Ok(tx) => {
                println!("{:?}", tx);
                data_plane.process_transaction(&tx, None, None)?;
            },
            Err(_) => println!("error: invalid input.")
        }
        prompt();
    }

    Ok(())
}
