use std::error::Error;

use crate::{request::Request, request_handler::RequestHandler};

pub fn from_path(path: &str, req_handler: &mut RequestHandler) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let req: Request = result?;
        if req_handler.handle(&req).is_err() {
            println!("[WARNING] Ignoring invalid request: {:#?}", req);
        };
    }

    Ok(())
}
