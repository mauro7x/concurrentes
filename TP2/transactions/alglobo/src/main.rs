use lib::{alglobo::AlGlobo, constants, file_logger::FileLogger};
use std::env;

// TODO: extract to config file
const REQS_FILENAME: &str = "./src/txs.csv";
const failed_reqs_filename: &str = "./src/output_logger.csv";

fn main() {
    let svc_name = env::var(constants::SVC_HOSTNAME).expect("SVC_HOSTNAME env variable undefined");
    let svc_port = env::var(constants::SVC_PORT).expect("SVC_PORT env variable undefined");
    let mut failed_requests_logger = FileLogger::new(failed_reqs_filename);
    let mut app: AlGlobo = AlGlobo::new(svc_name, svc_port);

    match app.run(REQS_FILENAME) {
        Ok(()) => (),
        Err(err) => panic!("error: {}", err),
    }
}
