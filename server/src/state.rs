use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub upload_directory: String,
    pub pool: SqlitePool,
}
