use crate::routes::{create_job, health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wraps the connections in a smart pointer
    let db_pool = Data::new(db_pool);

    // We then capture the connection from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // To add middlewares we need to use the `wrap` method on `APP`
            // Replaced `Logger::Default` with `TracingLogger`
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .route("/create_job", web::post().to(create_job))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
