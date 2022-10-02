// use crate refers to the current project.
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // Create a new Data layer with our Postgres connection.
    let connection = web::Data::new(connection);

    // Move capture closure's environment by value. The values' ownership is moved to the closure.
    // This is usually used when threading is involved.
    // Create a new server, configure endpoints, and data layer.
    // Start listening on our TcpListener.
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
