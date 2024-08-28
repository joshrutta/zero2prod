
use actix_web::{web, Responder, HttpResponse};
use diesel_async::AsyncPgConnection;
use crate::models::create_subscription;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<AsyncPgConnection>,
) -> impl Responder {
    let connection = &mut connection.clone();
    create_subscription(connection, &form.email, &form.name);
    HttpResponse::Ok()
}