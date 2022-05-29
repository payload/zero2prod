use axum::{extract::Form, http::StatusCode, Extension};
use sqlx::PgPool;

pub async fn subscribe(req: Form<SubscribeRequest>, db: Extension<PgPool>) -> StatusCode {
    let uuid = sqlx::types::Uuid::from_bytes(*uuid::Uuid::new_v4().as_bytes());

    println!("subscribe name={} email={}", req.name, req.email);
    match sqlx::query!(
        r#"insert into subscriptions (id, name, email, subscribed_at) values ($1, $2, $3, $4)"#,
        uuid,
        req.name,
        req.email,
        chrono::Utc::now()
    )
    .execute(&*db)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("subscribe failed: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscribeRequest {
    name: String,
    email: String,
}
