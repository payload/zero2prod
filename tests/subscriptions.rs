use std::sync::Arc;

use bitflips::Settings;
use reqwest::{self, RequestBuilder, StatusCode};
use sqlx::{prelude::*, PgConnection};

async fn spawn_app() -> (String, Arc<sqlx::PgConnection>, Settings) {
    let mut settings = bitflips::get_settings_expect();
    settings.app.port = 0;
    let init_state = bitflips::init(&settings).await;
    let endpoint = format!("http://{}/subscriptions", init_state.addr);
    let connection = init_state.database_connection.clone();
    tokio::spawn(bitflips::run(init_state));
    (endpoint, connection, settings)
}

#[tokio::test]
async fn subscribe_responds_ok_for_valid_form_data() {
    let (subscriptions, _, settings) = spawn_app().await;
    let mut connection = PgConnection::connect(&settings.database.connection_string()).await.expect("PgConnection::connect");

    let client = reqwest::Client::new();
    let valid_input = [
        (("name", "DenverCoder9"), ("email", "funny@valen.tine")),
        (("name", "funny@valen.tine"), ("email", "funny@valen.tine")),
    ];

    for input in valid_input {
        let response = client
            .post(&subscriptions)
            .form(&input)
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status(), "{:?}", input);

        let entry = sqlx::query!("SELECT email, name FROM subscriptions").fetch_one(&mut connection).await.expect("sqlx::query subscriptions");
        assert_eq!(entry.name, input.0.1, "name");
        assert_eq!(entry.email, input.1.1, "email");
    }
}

#[tokio::test]
async fn subscribe_responds_4xx_for_invalid_requests() {
    let (endpoint, ..) = spawn_app().await;
    let endpoint_ref = &endpoint;

    let request = || reqwest::Client::new().post(endpoint_ref);
    let check = |req: RequestBuilder| async {
        let msg = format!("{:?}", req);
        let response = req.send().await.unwrap();
        assert!(response.status().is_client_error(), "{:?}", msg);
    };

    check(request()).await;
    check(request().header("Context-Type", "application/x-www-form-urlencoded")).await;
    check(request().form(&[("name", "no email")])).await;
    check(request().form(&[("email", "no name")])).await;
    check(request().body("name=valid_from_data&email=but_bad_content_type")).await;
}
