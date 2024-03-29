use actix_web::{get, HttpResponse, Responder};

use crate::services::hello_service;

#[get("api/hello")]
pub async fn hello() -> impl Responder
{
    match hello_service::hello() {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(err_message) => HttpResponse::InternalServerError().body(err_message),
    }
}