use reqwest::{self, RequestBuilder, StatusCode};
use sqlx::PgPool;

struct TestApp {
    address: String,
    db_pool: PgPool,
}

impl TestApp {
    async fn new() -> Self {
        let mut settings = bitflips::get_settings_expect();
        settings.app.port = 0;
        let init_state = bitflips::init(&settings).await;
        let address = init_state.addr.clone();
        let db_pool = init_state.db_pool.clone();
        tokio::spawn(bitflips::run(init_state));
        Self { address, db_pool }
    }

    fn http(&self) -> reqwest::Client {
        reqwest::Client::new()
    }

    fn url(&self, path: &str) -> String {
        let address = &self.address;
        format!("http://{address}{path}")
    }

    fn db(&self) -> &PgPool {
        &self.db_pool
    }
}

#[tokio::test]
async fn subscribe_responds_ok_for_valid_form_data() {
    let app = TestApp::new().await;
    let valid_input = [
        (("name", "DenverCoder9"), ("email", "funny@valen.tine")),
        (("name", "funny@valen.tine"), ("email", "funny@valen.tine")),
    ];

    for input in valid_input {
        let response = app
            .http()
            .post(&app.url("/subscriptions"))
            .form(&input)
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status(), "{:?}", input);

        let entry = sqlx::query!("SELECT name, email FROM subscriptions")
            .fetch_one(app.db())
            .await
            .expect("sqlx::query subscriptions");
        assert_eq!(entry.name, input.0 .1, "name");
        assert_eq!(entry.email, input.1 .1, "email");
    }
}

#[tokio::test]
async fn subscribe_responds_4xx_for_invalid_requests() {
    let app = TestApp::new().await;

    let request = || app.http().post(app.url("/subscriptions"));
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
