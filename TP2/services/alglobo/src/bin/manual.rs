use lib::manual::run_manual_alglobo;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// ----------------------------------------------------------------------------

fn main() {
    pretty_env_logger::init();

    if let Err(err) = run_manual_alglobo() {
        error!("{}", err);
    }
}
