use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub workplace_type: String,
    pub address: String,
    pub employment_type: String,
    pub description: String,
    pub skills: String, // Verify this
    pub tools: String,
    pub min_salary: String,
    pub max_salary: String,
    pub salary_rate: String,
    pub bonuses: String,
    pub benefits: String,
    pub is_active: bool,
    pub agreement: bool,

    // recruiter info
    pub first_name: String,
    pub last_name: String,
    pub company_size: String,
    pub phone_number: String,
}

// Spans, like logs, have an associated level
// `info_span` creates a span at the level info
#[tracing::instrument(name = "Getting all jobs", skip(pool))]
pub async fn get_all_jobs(pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    tracing::info!("request_id {} -> ", request_id);
    let jobs = sqlx::query!("SELECT * FROM jobs;")
        .fetch_all(pool.get_ref())
        .await;

    tracing::info!(
        "request_id {} -> All jobs pull from the database",
        request_id
    );

    match jobs {
        Ok(jobs) => {
            let jobs: Vec<Job> = jobs
                .into_iter()
                .map(|job| Job {
                    id: job.id.as_hyphenated().to_string(),
                    title: job.title,
                    company: job.company,
                    workplace_type: job.workplace_type,
                    address: job.address,
                    employment_type: job.employment_type,
                    description: job.description,
                    skills: job.skills.unwrap(),
                    tools: job.tools.unwrap(),
                    min_salary: job.min_salary,
                    max_salary: job.max_salary,
                    salary_rate: job.salary_rate,
                    bonuses: job.bonuses.unwrap(),
                    benefits: job.benefits.unwrap(),
                    is_active: job.is_active,
                    agreement: job.agreement,
                    // Recruiter info
                    first_name: job.first_name,
                    last_name: job.last_name,
                    company_size: job.company_size,
                    phone_number: job.phone_number,
                })
                .collect();

            HttpResponse::Ok().json(jobs)
        }

        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Getting a single job", skip(pool))]
pub async fn get_single_job(job_id: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let id = Uuid::parse_str(&job_id).unwrap();

    tracing::info!("request_id {} -> ", request_id);
    let job = sqlx::query!(r#"SELECT * FROM jobs WHERE jobs.id = $1;"#, id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    println!("JOB: {:#?}", job);

    let job = Job {
        id: job.id.as_hyphenated().to_string(),
        title: job.title,
        company: job.company,
        workplace_type: job.workplace_type,
        address: job.address,
        employment_type: job.employment_type,
        description: job.description,
        skills: job.skills.unwrap(),
        tools: job.tools.unwrap(),
        min_salary: job.min_salary,
        max_salary: job.max_salary,
        salary_rate: job.salary_rate,
        bonuses: job.bonuses.unwrap(),
        benefits: job.benefits.unwrap(),
        is_active: job.is_active,
        agreement: job.agreement,
        // Recruiter info
        first_name: job.first_name,
        last_name: job.last_name,
        company_size: job.company_size,
        phone_number: job.phone_number,
    };

    HttpResponse::Ok().json(job)
}
