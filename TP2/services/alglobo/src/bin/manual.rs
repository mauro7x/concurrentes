use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    println!("Reading user input..");
    for line in stdin.lock().lines() {
        println!("READ: {}", line.unwrap());
    }
}