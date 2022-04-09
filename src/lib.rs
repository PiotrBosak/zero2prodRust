pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
use crate::routes::health_check;
use crate::routes::subscribe;
use actix_web::dev::Server;
use actix_web::HttpRequest;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use sqlx::PgConnection;
use sqlx::PgPool;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
