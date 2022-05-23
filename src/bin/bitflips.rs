#[tokio::main]
async fn main() -> hyper::Result<()> {
    let settings = bitflips::get_settings_expect();
    let init_state = bitflips::init(&settings).await;
    println!("Listening on {}", init_state.addr);
    bitflips::run(init_state).await?;
    Ok(())
}
