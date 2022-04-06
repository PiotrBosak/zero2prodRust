pub mod configuration;
pub mod routes;
pub mod startup;
use crate::routes::health_check;
use crate::routes::subscribe;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Responder};
use actix_web::HttpRequest;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
