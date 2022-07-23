use env_logger::Env;
use mythus::configuration::get_configuration;
use mythus::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logger Setup
    // `init` does call `set_logger`, so this is all we need to do
    // We print all logs at info-level or above
    // if RUST_LOG env variable is not set.

    // Remove env logger to replace it with tracing
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Falls back to print all span at info-level or above
    // if the RUST_LOG environment variable is not set.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(
        "mythus".into(),
        // Outputs the formatted span
        std::io::stdout,
    );

    // The `with` method provided by `SubscriberExt`, an extension
    // trait for `Subscriber` expose by `tracing_subscriber`

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // `set_global_default` can be used by applications to specify
    //what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber.");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgress");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
