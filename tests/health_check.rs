use reqwest::{self, StatusCode};

#[tokio::test]
async fn health_check() {
    tokio::spawn(bitflips::run());
    let response = reqwest::get("http://localhost:3000/health_check")
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(Some(0), response.content_length());
}
