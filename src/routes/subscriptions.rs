use axum::extract::Form;

///
pub async fn subscribe(req: Form<SubscribeRequest>) {
    println!("subscribe name={} email={}", req.name, req.email);
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscribeRequest {
    name: String,
    email: String,
}
