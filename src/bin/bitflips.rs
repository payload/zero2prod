use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    LogTracer::init().expect("Logging could not be set up.");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("bitflips".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Tracing could not be set up.");

    let settings = bitflips::get_settings_expect();
    let init_state = bitflips::init(&settings).await;
    log::info!("Listening on {}", init_state.addr);
    bitflips::run(init_state).await?;
    Ok(())
}
