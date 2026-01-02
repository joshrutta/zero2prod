use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use crate::routes::{health_check, subscribe, confirm, publish_newsletter};
use crate::email_client::EmailClient;
use crate::configuration::{DatabaseSettings, Settings};

pub struct Application {
    port: u16,
    server: Server
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        // Build an email client using configuration
        let sender_email = configuration.email_client.sender()
            .expect("Invalid sender email address");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout
        );
        // getting port from settings
        let address = format!("{}:{}", configuration.application.host, configuration.application.port);
        let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind to port");
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, 
            connection_pool, 
            email_client, 
            configuration.application.base_url
        )?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
    // better name that shows fn only returns when app is stopped
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// We need to define a wrapper type in order to retrieve the URL
// in the `subscribe` handler.
// Retrieval from the context, in actix-web, is type-based: using
// a raw `String` would expose us to conflicts.
pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    pg_pool: PgPool,
    email_client: EmailClient,
    base_url: String
) -> Result<Server, std::io::Error> {
    let pg_pool = web::Data::new(pg_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new( move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .route("/newsletters", web::post().to(publish_newsletter))
            .app_data(pg_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })            
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn get_connection_pool(database_configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_lazy_with(database_configuration.with_db())
}