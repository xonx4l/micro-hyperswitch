use axum::{ routing::{post,get},Router,http::status};
use std::sync::Arc;


async fn main() -> Result<() , Box<dyn std::error::Error>> {

    tracing_subscriber::registry()
    .init();
}