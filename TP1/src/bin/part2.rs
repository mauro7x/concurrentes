use actix::{Actor, Addr};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use lib::part2::tests::{Mensajito, MyActor};

struct AppState {
    app_name: String,
    my_actor: Addr<MyActor>,
}

// Temp
#[get("/test")]
async fn test(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    let addr = &data.my_actor;

    let msg = Mensajito { id: 34 };
    addr.try_send(msg).unwrap();

    format!("Hello {}!", app_name) // <- response with app_name
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Index page")
}

#[post("/request")]
async fn book() -> impl Responder {
    HttpResponse::Ok().body("Eventualmente un ID de request")
}

#[get("/request")]
async fn status() -> impl Responder {
    // query params?
    HttpResponse::Ok().body("El status de la request")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = MyActor::default().start();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
                my_actor: addr.clone(),
            })
            .service(test)
            .service(index)
            .service(book)
            .service(status)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
