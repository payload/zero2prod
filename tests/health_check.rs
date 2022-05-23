use reqwest::{self, StatusCode};

#[tokio::test]
async fn health_check_responds_ok_0_content_length() {
    let mut settings = bitflips::get_settings_expect();
    settings.app.port = 0;
    let init_state = bitflips::init(&settings).await;
    let health_check = format!("http://{}/health_check", init_state.addr);
    tokio::spawn(bitflips::run(init_state));

    let response = reqwest::get(health_check)
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(Some(0), response.content_length());
}
