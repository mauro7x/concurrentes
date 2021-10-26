//! File request parser and dispatcher

use std::error::Error;

use crate::part1::{request::Request, request_handler::RequestHandler};

/// Given a String representing a system file path and a RequestHandler
/// this method will parse the file (csv type required) into a Request struct and
/// call the handler in RequestHandler to process the request.

pub fn from_path(path: &str, req_handler: &mut RequestHandler) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let req: Request = result?;
        let req_clone: Request = req.clone();
        if req_handler.handle(req_clone).is_err() {
            println!("[WARNING] Ignoring invalid request: {:#?}", req);
        };
    }

    Ok(())
}
