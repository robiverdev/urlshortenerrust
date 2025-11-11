use actix_web::{App, HttpResponse, HttpServer, web};
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

// Generate random 6 char code
fn generate_code() -> String {
    use rand::Rng;
    // Base62 char set
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng(); // Random number generator

    // Generate 6 random chars
    (0..6)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len()); // Random index
            CHARSET[idx] as char // Convery byte to char
        })
        .collect() // Collect chars into String
}

// Macro that sets up async runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on localhost:4000");
    println!("Test code {}", generate_code()); // Test for generate_code

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

