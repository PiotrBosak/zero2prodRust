use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct Email(String);

#[derive(serde::Deserialize)]
struct Username(String);

#[derive(serde::Deserialize)]
pub struct FormatData {
    email: Email,
    name: Username,
}

pub async fn subscribe(_form: web::Form<FormatData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
