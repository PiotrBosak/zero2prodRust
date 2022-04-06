use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
