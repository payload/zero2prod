use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, prelude::*, EnvFilter, Registry};

/// Use with `init_tracing`
/// * `env_filter` - Usually `"info"` or `"debug"`
/// * `writer` - Usually `std::io::stdout` or `std::io::sink`
pub fn new_tracing_subscriber<W>(name: &str, env_filter: &str, writer: W) -> impl Subscriber
where
    W: for<'a> MakeWriter<'a> + 'static,
{
    return Registry::default()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter)))
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(name.into(), writer));
}

pub fn init_tracing(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Logging could not be set up.");
    tracing::subscriber::set_global_default(subscriber).expect("Tracing could not be set up.");
}
