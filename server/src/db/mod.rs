// TODO: add database table types here

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Media {
    pub id: String,
    pub filenmae: String,
    pub path: String,
    pub mime_type: String,
    pub uploaded_at: String,
    pub folder_id: Option<String>,
}