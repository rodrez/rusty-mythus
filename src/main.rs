use env_logger::Env;
use mythus::configuration::get_configuration;
use mythus::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logger Setup
    // `init` does call `set_logger`, so this is all we need to do
    // We print all logs at info-level or above
    // if RUST_LOG env variable is not set.

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgress");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
