use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Try getting the log level from RUST_LOG. If not possible then set it to the default
    // env_filter param.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Format logs into structured JSONs
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // Create span metadata storage
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Converts logs into trace events. Structured logs about program's execution.
    LogTracer::init().expect("Failed to set logger");

    // Set the global subscriber from a span storage. Can only be done once.
    set_global_default(subscriber).expect("Failed to set subscriber");
}
