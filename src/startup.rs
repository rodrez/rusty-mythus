use crate::email_client::EmailClient;
use crate::routes::{
    create_job, delete_job, get_all_jobs, get_single_job, health_check, subscribe, update_job,
};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{http, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // Wraps the connections in a smart pointer
    let db_pool = Data::new(db_pool);

    // Wraps the EmailClient
    let email_client = Data::new(email_client);

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
            .route("/v1/all_jobs", web::get().to(get_all_jobs))
            .route("/v1/single_job/{job_id}", web::get().to(get_single_job))
            .route("/v1/delete_job/{job_id}", web::post().to(delete_job))
            .route("/v1/update_job/{job_id}", web::post().to(update_job))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
