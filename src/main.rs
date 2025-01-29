use sqlx::postgres::PgPoolOptions;
use zero2prod::email_client::EmailClient;
use zero2prod::{configuration::get_configuration, startup::run};
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
    .connect_lazy_with(configuration.database.with_db());
    // Build an email client using configuration
    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email
    );
    // getting port from settings
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind to port");
    run(listener, connection_pool, email_client)?.await
}