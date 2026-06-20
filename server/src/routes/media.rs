use axum::{
    self, Json, Router, body::Body, extract::{Multipart, Path, State}, http::{HeaderMap, HeaderValue, StatusCode, header}, response::Response, routing::{get, post}
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{db::Media, state::AppState};

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
) -> Result<Response, StatusCode> {
    let media = sqlx::query_as::<_, Media>("SELECT * FROM media WHERE ID = ?")
        .bind(&id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // build response
    let bytes = tokio::fs::read(&media.path).await.map_err(|_| StatusCode::NOT_FOUND)?;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, &media.mime_type)
        .body(Body::from(bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
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

        sqlx::query("INSERT INTO media (id, filename, mime_type, path) values (? ? ? ?)")
            .bind(&id)
            .bind(&upload_filename)
            .bind("image/unknown")
            .bind(&path)
            .execute(&state.pool)
            .await
            .unwrap();

        return Ok(Json(PostUploadResponse {
            id,
            filename: upload_filename,
            path,
        }));
    }

    Err(StatusCode::BAD_REQUEST)
}
