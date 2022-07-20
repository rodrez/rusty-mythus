use actix_web::web::Form;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Skills {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tools {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct JobFormData {
    title: String,
    company: String,
    workplace_type: String,
    address: String,
    employment_type: String,
    description: String,
    skills: String, // Verify this
    tools: String,
    min_salary: String,
    max_salary: String,
    salary_rate: String,
    bonuses: String,
    benefits: String,
    is_active: bool,
}
// -d "title=Test&company=Test&workplace_type=Test&address=Test&employment_type=Test&description=Test&skills=Test&tools=Test&min_salary=Test&max_salary=Test&salary_rate=Test&bonuses=Test&benefits=Test&is_active=true&created_at=2020-01-01T00:00:00Z&updated_at=2020-01-01T00:00:00Z"
pub async fn create_job(form: Form<JobFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // Unique Identifier to trace issues
    let request_id = Uuid::new_v4();

    tracing::info!(
        "
        request_id {} -> 
        JOB DATA: {}, {}, {}, {}, {}, {}, {:?}, {:?}, {}, {}, {}, {}, {}, {}
        ",
        request_id,
        form.title,
        form.company,
        form.workplace_type,
        form.address,
        form.employment_type,
        form.description,
        form.skills,
        form.tools,
        form.min_salary,
        form.max_salary,
        form.salary_rate,
        form.bonuses,
        form.benefits,
        form.is_active
    );
    tracing::info!(
        "request_id {} -> Saving new job details in the database",
        request_id
    );

    match sqlx::query!(
        r#"
            INSERT INTO jobs (id, title, company, workplace_type, address, employment_type, description, skills, tools, min_salary, max_salary, salary_rate, bonuses, benefits, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            "#,
        Uuid::new_v4(),
        form.title,
        form.company,
        form.workplace_type,
        form.address,
        form.employment_type,
        form.description,
        form.skills,
        form.tools,
        form.min_salary,
        form.max_salary,
        form.salary_rate,
        form.bonuses,
        form.benefits,
        Utc::now(),
        Utc::now(),
        true,
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} -> New job have been saved.",
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
