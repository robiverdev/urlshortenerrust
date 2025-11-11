use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize}; // Automatically convert structs to/from JSON
use std::collections::HashMap; // "Database"
use std::sync::RwLock; // Allows multiple readers/one writer at a time

// Request
#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

// Response
#[derive(Serialize)]
struct ShortenResponse {
    short_code: String,
    short_url: String,
}

// App state shared across all requests
struct AppState {
    urls: RwLock<HashMap<String, String>>,
}

// Macro that sets up async runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on localhost:4000");

    // Create HTTP server
    HttpServer::new(|| {
        // Create new app instance for each worker thread
        App::new().route(
            "/",
            // Handler GET requests to "/" with asycc closure
            web::get().to(|| async { HttpResponse::Ok().body("URL Shortener Homepage") }),
        )
    })
    .bind(("localhost", 4000))? // Bind to address, ? - error propagation
    .run() // Start server
    .await // Wait for server to shutdown
}
