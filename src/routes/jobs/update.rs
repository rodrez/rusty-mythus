use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

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

    println!("JOB ID: {:#?}", job_id);
    println!("FORM: {:#?}", form);

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

    HttpResponse::Ok().content_type("application/json").finish()
    // todo:: Update function to patch
}
