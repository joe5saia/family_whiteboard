mod models;
mod handlers;
mod database;
mod websocket;

use axum::{
    extract::Extension,
    http::Method,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use std::sync::Arc;
use tracing::info;

use crate::database::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize database
    let database = Arc::new(Database::new().await?);
    
    // Run migrations
    database.migrate().await?;
    
    // Setup CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);
    
    // Build application routes
    let app = Router::new()
        // API routes
        .route("/api/todos", get(handlers::get_todos))
        .route("/api/todos", post(handlers::create_todo))
        .route("/api/todos/:id", put(handlers::update_todo))
        .route("/api/todos/:id/toggle", put(handlers::toggle_todo))
        .route("/api/todos/:id", delete(handlers::delete_todo))
        // WebSocket route
        .route("/ws", get(websocket::websocket_handler))
        // Static file serving for frontend
        .nest_service("/", ServeDir::new("../"))
        .layer(Extension(database))
        .layer(cors);
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}