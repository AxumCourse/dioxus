use sqlx::PgExecutor;

use crate::model;

pub async fn create(e: impl PgExecutor<'_>, message: &model::Message) -> sqlx::Result<&str> {
    sqlx::query("INSERT INTO messages (id, content, password, dateline) VALUES ($1, $2, $3, $4)")
        .bind(&message.id)
        .bind(&message.content)
        .bind(&message.password)
        .bind(&message.dateline)
        .execute(e)
        .await?;

    Ok(&message.id)
}

pub async fn get(e: impl PgExecutor<'_>, id: &str) -> sqlx::Result<Option<model::Message>> {
    sqlx::query_as("SELECT id, content, password, dateline FROM messages WHERE id = $1")
        .bind(id)
        .fetch_optional(e)
        .await
}

pub async fn del(e: impl PgExecutor<'_>, id: &str) -> sqlx::Result<u64> {
    let aff = sqlx::query("DELETE FROM messages WHERE id = $1")
        .bind(id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}
