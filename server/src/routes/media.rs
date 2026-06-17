use axum::{
    self, Json, Router, extract::{Multipart, State, Path}, http::StatusCode, routing::{post, get}
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::Row;

use crate::state::AppState;

#[derive(Serialize)]
struct PostUploadResponse {
    id: String,
    filename: String,
    path: String,
}

#[derive(Serialize)]
struct GetMediaResponse {
    id: String,
    filename: String,
    // add body?
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/upload", post(upload_media))
        .route("/{id}", get(get_media))

    //TODO: /media to get all
    //TODO: /media/{id} delete to delete media
}

async fn get_media(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetMediaResponse>, StatusCode> {
    let row = sqlx::query(
        "SELECT * FROM media WHERE ID = ?",
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    match row {
        Some(row) => {
            let id: String = row.get("id");
            let filename: String = row.get("filename");
            let mime_type: String = row.get("mime_type");
            let path: String = row.get("path");

            // build response
            let bytes = tokio::fs::read(&path).await.unwrap();
            return Err(StatusCode::NOT_IMPLEMENTED)
        }
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    }
}

async fn upload_media(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<PostUploadResponse>, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let upload_filename = field.file_name().unwrap_or("upload.bin").to_string();
        let bytes = field.bytes().await.unwrap();
        let id = Uuid::new_v4().to_string();
        let path = format!("{}/{}_{}", state.upload_directory, id, upload_filename);

        tokio::fs::write(&path, bytes).await.unwrap();

        sqlx::query(
            "INSERT INTO media (id, filename, mime_type, path) values (? ? ? ?)"
        )
        .bind(&id)
        .bind(&upload_filename)
        .bind("image/unknown")
        .bind(&path)
        .execute(&state.pool)
        .await
        .unwrap();

        return Ok(Json(PostUploadResponse {
            id: "yo".to_string(),
            filename: "yo".to_string(),
            path: "yo".to_string(),
        }))
    }

    Err(StatusCode::BAD_REQUEST)
}
