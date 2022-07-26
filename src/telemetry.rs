use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// Compose multiple layers into a `tracing`'s subscriber
//
// # Implementation Notes
//
// We are using are using `impl Subscriber` as a return type to avoid having to
// spell out the actual type of the returned subscriber, which is
// quite complex
//
// We need to explicitly called out that the returned subscriber is
// "Send" and "Sync" to make it possible to pass it to "init_subscriber" later on
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Falls back to print all span at info-level or above
    // if the RUST_LOG environment variable is not set.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // The `with` method provided by `SubscriberExt`, an extension
    // trait for `Subscriber` expose by `tracing_subscriber`
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        // Outputs the formatted span
        .with(formatting_layer)
}

// Register subscriber as a global default to to process span data.
//
// This function should be called only once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirecting all `log`'s event to our subscriber
    LogTracer::init().expect("Failed to set logger.");

    // `set_global_default` can be used by applications to specify
    //what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");
}
