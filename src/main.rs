use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use diesel_async::pooled_connection::bb8::Pool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let database_url = configuration.database.connection_string();
    let connection_manager = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().build(connection_manager).await.unwrap();
    
    // getting port from settings now!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, pool)?.await
}
