#[tokio::main]
async fn main() -> hyper::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,bitflips"))
        .init();
    let settings = bitflips::get_settings_expect();
    let init_state = bitflips::init(&settings).await;
    log::info!("Listening on {}", init_state.addr);
    bitflips::run(init_state).await?;
    Ok(())
}
