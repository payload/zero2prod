use axum::{extract::Form, http::StatusCode, Extension};
use sqlx::PgPool;
use tracing::Instrument;

pub async fn subscribe(req: Form<SubscribeRequest>, db: Extension<PgPool>) -> StatusCode {
    let uuid = sqlx::types::Uuid::from_bytes(*uuid::Uuid::new_v4().as_bytes());
    let span = tracing::info_span!("subscribe", %uuid, ?req, name = %req.name, email = %req.email);
    let _guard = span.enter();

    match sqlx::query!(
        r#"insert into subscriptions (id, name, email, subscribed_at) values ($1, $2, $3, $4)"#,
        uuid,
        req.name,
        req.email,
        chrono::Utc::now()
    )
    .execute(&*db)
    .instrument(tracing::info_span!("subscribing"))
    .await
    {
        Ok(_) => {
            tracing::info!("subscribe done {uuid}");
            StatusCode::OK
        }
        Err(err) => {
            if let sqlx::Error::Database(pg_err) = err {
                tracing::error!("subscribe error: {}", pg_err.message());
            } else {
                tracing::error!("subscribe error {err:?}");
            }
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscribeRequest {
    name: String,
    email: String,
}
