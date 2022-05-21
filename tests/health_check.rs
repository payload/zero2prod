use reqwest::{self, StatusCode};

#[tokio::test]
async fn health_check() {
    let (listener, addr) = bitflips::bind_localhost("0");
    tokio::spawn(bitflips::run(listener));
    let response = reqwest::get(format!("http://{addr}/health_check"))
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(Some(0), response.content_length());
}
