use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::routes::get_single_job;

#[derive(Deserialize, Serialize, Debug)]
pub struct Job {
    pub title: Option<String>,
    pub company: Option<String>,
    pub workplace_type: Option<String>,
    pub address: Option<String>,
    pub employment_type: Option<String>,
    pub description: Option<String>,
    pub skills: Option<String>, // Verify this
    pub tools: Option<String>,
    pub min_salary: Option<String>,
    pub max_salary: Option<String>,
    pub salary_rate: Option<String>,
    pub bonuses: Option<String>,
    pub benefits: Option<String>,
    pub is_active: Option<bool>,
    pub agreement: Option<bool>,

    // recruiter info
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_size: Option<String>,
    pub phone_number: Option<String>,
}

pub async fn update_job(
    job_id: web::Path<String>,
    form: web::Form<Job>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();

    tracing::info!(
        "
        request_id {} ->
        JOB DATA: title={:#?}, company={:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}, {:#?}
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
        form.is_active,
        form.first_name,
        form.last_name,
        form.company_size,
        form.phone_number,
    );
    tracing::info!(
        "request_id {} -> Saving new job details in the database",
        request_id
    );

    let job = sqlx::query!(
        // This is how we do optional updates of fields without needing a separate query for each.
        // PostgreSQL
        // todo: Need to create a dynamic function that generates a string based of the passed parameters
        r#"
            UPDATE jobs
            SET title = coalesce($1, "jobs".title),
                company = coalesce($2, "jobs".company),
                workplace_type = coalesce($3, "jobs".workplace_type),
                address = coalesce($4, "jobs".address),
                employment_type = coalesce($5, "jobs".employment_type),
                description = coalesce($6, "jobs".description),
                skills = coalesce($7, "jobs".skills),
                tools = coalesce($8, "jobs".tools),
                min_salary = coalesce($9, "jobs".min_salary),
                max_salary = coalesce($10, "jobs".max_salary),
                salary_rate = coalesce($11, "jobs".salary_rate),
                bonuses = coalesce($12, "jobs".bonuses),
                benefits = coalesce($13, "jobs".benefits),
                is_active = coalesce($14, "jobs".is_active),
                first_name = coalesce($15, "jobs".first_name),
                last_name = coalesce($16, "jobs".last_name),
                company_size = coalesce($17, "jobs".company_size),
                phone_number = coalesce($18, "jobs".phone_number),
                agreement = coalesce($19, "jobs".agreement)
            WHERE id = $20
        "#,
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
        form.is_active,
        form.first_name,
        form.last_name,
        form.company_size,
        form.phone_number,
        form.agreement,
        Uuid::parse_str(&job_id).unwrap()
    )
    .execute(pool.as_ref())
    .await;

    match job {
        Ok(_) => {
            tracing::info!("request_id {} ->Job has been updated.", request_id);

            get_single_job(job_id, pool).await
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

    // todo:: Update function to patch
}
