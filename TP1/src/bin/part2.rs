use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind("127.0.0.1:8080")?;
    println!("Listening on port {:?}", 8080);

    server.run().await
}
