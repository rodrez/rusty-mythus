use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_job(job_id: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    // Unique Identifier to trace issues
    let request_id = Uuid::new_v4();

    tracing::info!(
        "
           request_id {} -> 
           DELETE JOB: {} 
           ",
        request_id,
        job_id
    );
    let id = Uuid::parse_str(&job_id).unwrap();

    match sqlx::query!("DELETE FROM jobs WHERE id=$1", id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => {
            tracing::info!("request_id {} -> Job have been deleted.", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} -> Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
