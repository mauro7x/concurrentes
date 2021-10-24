// use actix::Addr;
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// use lib::part2::{
//     airlines,
//     dispatcher::{HandleBook, WebServiceDispatcher},
// };
// use lib::{paths, request::Request};

// use std::collections::HashMap;

// struct AppState {
//     airlines: HashMap<String, Addr<WebServiceDispatcher>>,
//     req_id: u64,
// }

// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Index page")
// }

// #[post("/request")]
// async fn book(req: web::Json<Request>, data: web::Data<AppState>) -> impl Responder {
//     let airlines = &data.airlines;
//     let req_id = data.req_id;

//     println!("[REQ] {:#?}", req);
//     match airlines.get(&req.airline) {
//         Some(airline_dispatcher) => {
//             let msg = HandleBook {
//                 req_id: data.req_id,
//             };
//             airline_dispatcher.try_send(msg); // TODO handle result incorrect
//         }
//         None => {
//             println!("Airline {} does not exist", &req.airline)
//             // TODO Error response
//         }
//     }
//     HttpResponse::Ok().body(format!("Request id: {}", req_id))
// }

// #[get("/request")]
// async fn status() -> impl Responder {
//     // query params?
//     HttpResponse::Ok().body("El status de la request")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let airlines = airlines::from_path(paths::AIRLINES_CONFIG).expect("[CRITICAL]");

//     HttpServer::new(move || {
//         let airlines_cln = airlines.clone();

//         App::new()
//             .data(AppState {
//                 req_id: 0,
//                 airlines: airlines_cln,
//             })
//             .service(index)
//             .service(book)
//             .service(status)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

fn main() {}
