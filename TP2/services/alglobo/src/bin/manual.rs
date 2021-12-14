use lib::manual::run_manual_alglobo;

// ----------------------------------------------------------------------------

fn main() {
    if let Err(err) = run_manual_alglobo() {
        println!("[ERROR] {}", err);
    }
}
