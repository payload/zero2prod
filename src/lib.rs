use axum::routing::*;
use std::net::SocketAddr;

pub async fn run() -> hyper::Result<()> {
    let app = Router::new().route("/health_check", get(health_check));
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    server.await
}

// I am here for responding with 200 OK
async fn health_check() {}
