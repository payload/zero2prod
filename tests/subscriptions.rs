use bitflips::{init_tracing, new_tracing_subscriber, DatabaseSettings};
use once_cell::sync::Lazy;
use reqwest::{self, RequestBuilder, StatusCode};
use sqlx::{prelude::*, Connection, PgConnection, PgPool};
use tracing_subscriber::fmt::TestWriter;

static TRACING: Lazy<()> = Lazy::new(|| {
    init_tracing(new_tracing_subscriber("test", "debug", TestWriter::new()));
});

struct TestApp {
    address: String,
    db_pool: PgPool,
}

impl TestApp {
    async fn new() -> Self {
        Lazy::force(&TRACING);

        let mut settings = bitflips::get_settings_expect();
        settings.database.database_name = format!("test:{}", uuid::Uuid::new_v4());
        settings.app.port = 0;
        init_database(&settings.database).await;
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

async fn init_database(db_settings: &DatabaseSettings) {
    let mut conn = PgConnection::connect(&db_settings.connection_string_no_db())
        .await
        .expect("postgres connection");
    let create_database = format!(r#"CREATE DATABASE "{}";"#, db_settings.database_name);
    conn.execute(create_database.as_str())
        .await
        .expect(&create_database);

    let pool = PgPool::connect(&db_settings.connection_string())
        .await
        .expect("database connection");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("database migrations");
}

#[tokio::test]
async fn subscribe_responds_ok_for_valid_form_data() {
    let app = TestApp::new().await;
    let valid_input = [
        (("name", "DenverCoder9"), ("email", "funny@valen.tine")),
        (("name", "looks@like.email"), ("email", "is@an.email")),
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

        let entry = sqlx::query!(
            "SELECT name, email FROM subscriptions WHERE email=$1",
            input.1 .1
        )
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
