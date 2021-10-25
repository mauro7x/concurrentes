use std::error::Error;

use crate::part1::{request::Request, request_handler::RequestHandler};

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
