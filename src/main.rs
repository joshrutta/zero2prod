use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use diesel_async::{AsyncConnection, AsyncPgConnection};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = AsyncPgConnection::establish(
        &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    
    // getting port from settings now!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
