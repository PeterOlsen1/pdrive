use crate::db::Media;

pub async fn get_media_by_id(pool: sqlx::SqlitePool, id: String) -> Result<Option<Media>, sqlx::Error> {
    let media = sqlx::query_as::<_, Media>("SELECT * FROM media WHERE ID = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await;

    media
}
