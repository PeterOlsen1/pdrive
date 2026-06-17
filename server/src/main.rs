mod routes;
mod state;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

use crate::{routes::health::handle_health, state::AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let upload_directory = env::var("UPLOAD_DIRECTORY").expect("UPLOAD_DIRECTORY must be set");

    tokio::fs::create_dir_all(&upload_directory)
        .await
        .expect("Failed to create upload directory");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    println!("Running migrations");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let state = AppState{
        upload_directory: upload_directory,
        pool,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello from PDrive" }))
        .route("/health", get(handle_health))
        .nest("/media", routes::media::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
