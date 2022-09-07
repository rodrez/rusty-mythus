use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
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
}

// Spans, like logs, have an associated level
// `info_span` creates a span at the level info
#[tracing::instrument(name = "Getting all jobs", skip(pool))]
pub async fn get_jobs(pool: web::Data<PgPool>) -> HttpResponse {
    let jobs = sqlx::query!("SELECT * FROM jobs;")
        .fetch_all(pool.get_ref())
        .await;

    match jobs {
        Ok(jobs) => {
            let jobs: Vec<Job> = jobs
                .into_iter()
                .map(|job| Job {
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
                })
                .collect();
            println!("JOBS: {:?}", jobs);
            HttpResponse::Ok().json(jobs)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
