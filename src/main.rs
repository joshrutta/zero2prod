use sqlx::{Connection, PgPool};
use zero2prod::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    // getting port from settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind to random port");
    run(listener, connection_pool)?.await
}