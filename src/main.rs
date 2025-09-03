use axum::{ routing::{post,get},Router,http::status};
use std::sync::Arc;


async fn main() -> Result<() , Box<dyn std::error::Error>> {

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database = Database::new().await?;
    let shared_state = Arc::new(database);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/payments", post(payment_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("🚀 Micro Hyperswitch server starting on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}