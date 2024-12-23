use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string().expose_secret()
        )
        .await
        .expect("Failed to connect to Postgres.");
    // getting port from settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind to random port");
    run(listener, connection_pool)?.await
}