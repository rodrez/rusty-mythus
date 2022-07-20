use crate::routes::{create_job, health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // Wraps the connections in a smart pointer
    let connection = web::Data::new(connection);

    // We then capture the connection from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // To add middlewares we need to use the `wrap` method on `APP`
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .route("/create_job", web::post().to(create_job))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
