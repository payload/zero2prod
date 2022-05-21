use reqwest::{self, RequestBuilder, StatusCode};

#[tokio::test]
async fn subscribe_responds_ok_for_valid_form_data() {
    let (listener, addr) = bitflips::bind_localhost("0");
    tokio::spawn(bitflips::run(listener));
    let client = reqwest::Client::new();
    let valid_input = [
        (("name", "DenverCoder9"), ("email", "funny@valen.tine")),
        (("name", "funny@valen.tine"), ("email", "funny@valen.tine")),
    ];

    for input in valid_input {
        let response = client
            .post(format!("http://{addr}/subscriptions"))
            .form(&input)
            .send()
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, response.status(), "{:?}", input);
    }
}

#[tokio::test]
async fn subscribe_responds_4xx_for_invalid_requests() {
    let (listener, addr) = bitflips::bind_localhost("0");
    tokio::spawn(bitflips::run(listener));

    let request = || reqwest::Client::new().post(format!("http://{addr}/subscriptions"));
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
