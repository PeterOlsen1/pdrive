use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn handle_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
    })
}