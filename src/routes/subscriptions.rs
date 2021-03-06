use actix_web::web::Form;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // Unique Identifier to trace issues
    let request_id = Uuid::new_v4();

    // tracing::info!(
    //     "request_id {} -> Adding '{}' '{}' as a new subscriber.",
    //     request_id,
    //     form.email,
    //     form.name
    // );
    // tracing::info!(
    //     "request_id {} ->Saving new subscriber details in the database",
    //     request_id
    // );

    // Spans, like logs, have an associated level
    // `info_span` creates a span at the level info
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    // Using enter for async function is not good.
    let _request_span_guard = request_span.enter();

    // We do not use `.enter` as `instrument` takes care of
    // it at the right time
    let query_span = tracing::info_span!("Saving new subscriber details in database.");

    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} -> New subscriber details have been saved.",
                request_id
            );
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
