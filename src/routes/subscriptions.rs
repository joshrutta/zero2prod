
use actix_web::{web, Responder, HttpResponse};
use diesel_async::AsyncPgConnection;
use crate::models::create_subscription;
use diesel_async::pooled_connection::bb8::Pool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection_pool: web::Data<Pool<AsyncPgConnection>>,
) -> impl Responder {
    let connection_pool = connection_pool.get_ref();
    create_subscription(connection_pool, &form.email, &form.name).await;
    HttpResponse::Ok()
}