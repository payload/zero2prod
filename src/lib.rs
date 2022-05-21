use axum::{extract::Form, routing::*};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> hyper::Result<()> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    server.await
}

/// bind 127.0.0.1:{port} and returns listener and local addr as string
/// else panics
pub fn bind_localhost(port: &str) -> (TcpListener, String) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    (listener, addr)
}

/// responding with 200 OK
async fn health_check() {}

///
async fn subscribe(req: Form<SubscribeRequest>) {
    println!("subscribe name={} email={}", req.name, req.email);
}

#[derive(serde::Deserialize, Debug)]
struct SubscribeRequest {
    name: String,
    email: String,
}
