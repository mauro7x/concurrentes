//! Manual service: Built for manual payment retries. DataPlane used underneath.

use std::io::{self};

use crate::{data_plane::DataPlane, types::common::BoxResult};

// ----------------------------------------------------------------------------

fn prompt() {
    let expected_format = "tx_id,cbu,airline_cost,hotel_cost";
    println!(
        "\nPlease enter the transaction to retry: (format: {})\n",
        expected_format
    );
}

/// Execute retry logic.
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
            }
            Err(_) => println!("Error: invalid input."),
        }
        prompt();
    }

    Ok(())
}
