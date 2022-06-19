#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = bitflips::new_tracing_subscriber("bitflips", "info", std::io::stdout);
    bitflips::init_tracing(subscriber);

    let settings = bitflips::get_settings_expect();
    let init_state = bitflips::init(&settings).await;
    log::info!("Listening on {}", init_state.addr);
    bitflips::run(init_state).await?;
    Ok(())
}
