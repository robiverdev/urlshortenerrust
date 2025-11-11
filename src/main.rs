use actix_web::{web,App,HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on localhost:4000");

    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| async {
            HttpResponse::Ok().body("URL Shortener / route")
        }))
    }).bind(("localhost", 4000))?.run().await
}
