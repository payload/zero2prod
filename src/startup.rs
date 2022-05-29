use axum::{routing::*, Extension};
use std::net::TcpListener;

use crate::{routes::*, *};

pub async fn run(state: InitState) -> hyper::Result<()> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(state.db_pool));
    let server = axum::Server::from_tcp(state.listener)?.serve(app.into_make_service());
    server.await
}

pub async fn init(settings: &Settings) -> InitState {
    let (listener, addr) = bind_localhost(settings.app.port);
    let connection_string = settings.database.connection_string();
    let db_pool = sqlx::PgPool::connect(&connection_string)
        .await
        .expect("PgPool::connect");
    InitState {
        listener,
        addr,
        db_pool,
    }
}

pub struct InitState {
    pub listener: TcpListener,
    pub addr: String,
    pub db_pool: sqlx::PgPool,
}

/// bind 127.0.0.1:{port} and returns listener and local addr as string
/// else panics
pub fn bind_localhost(port: u16) -> (TcpListener, String) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    (listener, addr)
}
