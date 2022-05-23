use std::sync::Arc;

use axum::{extract::Form, Extension};
use sqlx::{ prelude::*, PgConnection};

///
pub async fn subscribe(req: Form<SubscribeRequest>, conn: Extension<Arc<PgConnection>>) {
    let uuid = sqlx::types::Uuid::from_bytes(*uuid::Uuid::new_v4().as_bytes());

    println!("subscribe name={} email={}", req.name, req.email);
    let x = sqlx::query!(
        r#"insert into subscriptions (id, email, name, subscribed_at) values ($1, $2, $3, $4)"#,
        uuid,
        req.email,
        req.name,
        chrono::Utc::now()
    );
    println!("{:?}", x);
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscribeRequest {
    name: String,
    email: String,
}
