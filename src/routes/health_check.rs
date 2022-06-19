use tracing::*;

#[tracing::instrument]
pub async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
