use mythus::configuration::get_configuration;
use mythus::startup::run;
use mythus::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("mythus".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // All configs come from `configuration.rs` and `configuration.yml`
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Exposed the secret
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to postgres");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await?;
    Ok(())
}
