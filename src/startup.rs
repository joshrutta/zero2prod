use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;

use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    connection_pool: Pool<AsyncPgConnection>,
) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new( move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}