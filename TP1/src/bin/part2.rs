use actix::{Actor, Addr};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use lib::part2::{
    errors::*,
    request::RawRequest,
    request_handler::{HandleRequest, RequestHandler},
};

struct ServerState {
    request_handler: Addr<RequestHandler>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Index page")
}

#[post("/request")]
async fn book(raw_request: web::Json<RawRequest>, state: web::Data<ServerState>) -> impl Responder {
    let request_handler = &state.request_handler;
    let msg = HandleRequest {
        raw_request: raw_request.clone(),
    };

    match request_handler.send(msg).await {
        Ok(Ok(req_id)) => HttpResponse::Created().body(req_id),
        Ok(Err(HandlerError::AirlineNotFound)) => {
            HttpResponse::NotFound().body(format!("Airline {} not found", raw_request.airline))
        }
        Ok(Err(HandlerError::AirlineUnavailable)) => HttpResponse::NotFound().body(format!(
            "Airline {} not available, try later",
            raw_request.airline
        )),
        Ok(Err(HandlerError::HotelUnavailable)) => {
            HttpResponse::NotFound().body("Hotel not available, try later")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", err))
        }
    }
}

#[get("/request")]
async fn status() -> impl Responder {
    // query params?
    HttpResponse::Ok().body("El status de la request")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let request_handler = RequestHandler::new().start();

    let server = HttpServer::new(move || {
        App::new()
            .data(ServerState {
                request_handler: request_handler.clone(),
            })
            .service(index)
            .service(book)
            .service(status)
    })
    .bind(format!("127.0.0.1:{}", port))?;
    println!("Listening on port {}", port);

    server.run().await
}
