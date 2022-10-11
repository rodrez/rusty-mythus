use mythus::configuration::get_configuration;
use mythus::email_client::EmailClient;
use mythus::startup::run;
use mythus::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("mythus".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // All configs come from `configuration.rs` and `configuration.yml`
    let configuration = get_configuration().expect("Failed to read configuration.");

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender address");

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    // Exposed the secret
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
