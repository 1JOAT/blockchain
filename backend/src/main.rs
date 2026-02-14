mod blockchain;
mod api;
mod storage;
mod errors;
mod handlers;

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::http::HeaderValue;
use tower_http::cors::{CorsLayer, Any};

use blockchain::Blockchain;
use api::{AppState, create_router};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let blockchain_file = std::env::var("BLOCKCHAIN_FILE").unwrap_or_else(|_| "blockchain.json".to_string());
    let difficulty: usize = std::env::var("DIFFICULTY")
        .unwrap_or_else(|_| "2".to_string())
        .parse()
        .unwrap_or(2);
    let allowed_origins = std::env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string());

    // Parse allowed origins for CORS
    let cors_origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| HeaderValue::from_str(s).unwrap())
        .collect();

    // Try to load existing blockchain, or create new one
    let blockchain = match Blockchain::load_from_file(&blockchain_file) {
        Ok(mut loaded) => {
            // Update difficulty if changed
            loaded.difficulty = difficulty;
            println!(" Loaded existing blockchain with {} blocks", loaded.chain.len());
            loaded
        },
        Err(e) => {
            println!(" Creating new blockchain: {}", e);
            Blockchain::new_with_difficulty(difficulty)
        }
    };

    let blockchain = Arc::new(Mutex::new(blockchain));

    let app_state = AppState {
        blockchain: blockchain.clone(),
    };

    // Configure CORS - allow specific origins for security
    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = create_router()
        .with_state(app_state)
        .layer(cors);

    let app = app.route("/", axum::routing::get(|| async {
        "Blockchain API Server - Use /blocks, /transaction, /mine, etc."
    }));

    let addr = format!("{}:{}", host, port);
    println!(" Blockchain server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener,app).await.unwrap();
}
