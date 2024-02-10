use actix_web::HttpResponse;

use crate::services::hello_service;

pub async fn hello() -> HttpResponse
{
    match hello_service::hello() {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(err_message) => HttpResponse::InternalServerError().body(err_message)
    }
}