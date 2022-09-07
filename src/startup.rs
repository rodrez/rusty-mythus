use crate::routes::{create_job, get_jobs, health_check, subscribe};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{http, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wraps the connections in a smart pointer
    let db_pool = Data::new(db_pool);

    // We then capture the connection from the surrounding environment
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            // To add middlewares we need to use the `wrap` method on `APP`
            // Replaced `Logger::Default` with `TracingLogger`
            .wrap(cors)
            .wrap(TracingLogger::default())
            .route("/v1/health_check", web::get().to(health_check))
            .route("/v1/subscribe", web::post().to(subscribe))
            .route("/v1/create_job", web::post().to(create_job))
            .route("/v1/get_jobs", web::get().to(get_jobs))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
