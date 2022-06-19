use axum::{extract::Form, http::StatusCode, Extension};
use sqlx::PgPool;

#[tracing::instrument(skip_all, ret, fields(
    request_id = %uuid::Uuid::new_v4(),
    request = ?*req,
))]
pub async fn subscribe(req: Form<SubscribeRequest>, db: Extension<PgPool>) -> StatusCode {
    match db_insert_subscription(&req, &*db).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip_all, ret, err)]
async fn db_insert_subscription(
    request: &SubscribeRequest,
    db: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"insert into subscriptions (id, name, email, subscribed_at) values ($1, $2, $3, $4)"#,
        sqlx_uuid(),
        request.name,
        request.email,
        chrono::Utc::now()
    )
    .execute(db)
    .await?;
    Ok(())
}

fn sqlx_uuid() -> sqlx::types::Uuid {
    sqlx::types::Uuid::from_bytes(*uuid::Uuid::new_v4().as_bytes())
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscribeRequest {
    name: String,
    email: String,
}
