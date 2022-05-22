use axum::routing::*;
use std::net::TcpListener;

use crate::routes::*;

pub async fn run(listener: TcpListener) -> hyper::Result<()> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    server.await
}
