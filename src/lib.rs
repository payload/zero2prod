mod configuration;
mod routes;
mod startup;

pub use startup::run;

use std::net::TcpListener;

/// bind 127.0.0.1:{port} and returns listener and local addr as string
/// else panics
pub fn bind_localhost(port: &str) -> (TcpListener, String) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    (listener, addr)
}
