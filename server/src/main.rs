mod routes;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use sqlx::sqlite::SqlitePoolOptions;

use crate::routes::health::handle_health;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello from PDrive" }))
        .route("/health", get(handle_health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}