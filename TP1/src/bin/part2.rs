extern crate actix;

use actix::prelude::*;
use lib::{
    actors::airlines::Airlines,
    actors::{
        airlines,
        logger::{from_config, LogMessage, Logger},
        request::{IncommingRequest, SystemRequest},
    },
    config::GeneralConfig,
    paths,
};

struct FileParser {
    path: String,
    logger_addr: Addr<Logger>,
    req_handler_addr: Addr<RequestHandler>,
}

impl Actor for FileParser {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let mut rdr = csv::Reader::from_path(&(self.path)).expect("msg");
        for result in rdr.deserialize() {
            let req: IncommingRequest = result.expect("[ERR] Parseando el request");
            self.logger_addr.do_send(LogMessage(
                "[INFO] Parsed Request in File Parser".to_string(),
            ));
            self.req_handler_addr.do_send(req);
        }
    }
}

struct RequestHandler {
    next_id: u32,
    airlines: Airlines,
}
impl Actor for RequestHandler {
    type Context = Context<Self>;
}

impl Handler<IncommingRequest> for RequestHandler {
    type Result = ();

    fn handle(&mut self, msg: IncommingRequest, _: &mut Context<Self>) {
        // OJO CON BLOQUEAR EL MAIN LOOP
        println!(
            "Hola, tengo que usar la aerolinea: {}, de {} hasta {}",
            msg.airline, msg.origin, msg.destiny,
        );
        let airline = self.airlines.get(&msg.airline).unwrap();
        let req_id = self.next_id;
        airline.do_send(SystemRequest {
            id: req_id,
            origin: msg.origin,
            destiny: msg.destiny,
            package: msg.package,
        });
        self.next_id += 1;
    }
}
fn main() {
    let system = System::new();
    let GeneralConfig { logger_config } =
        GeneralConfig::from_path(paths::GENERAL).expect("[CRITICAL] GlobalConfig error");

    let _addr = system.block_on(async {
        let logger_addr = from_config(logger_config)
            .expect("[CRITICAL] Logger initialization error")
            .start();

        let airlines = airlines::from_path(paths::AIRLINES_CONFIG, logger_addr.clone())
            .expect("[CRITICAL] Airlines initialization error");

        let req_handler_addr = RequestHandler {
            next_id: 0,
            airlines,
        }
        .start();

        let _ = FileParser {
            logger_addr,
            req_handler_addr,
            path: paths::REQUESTS.to_string(),
        }
        .start();
    });

    let _ = system.run();
}
