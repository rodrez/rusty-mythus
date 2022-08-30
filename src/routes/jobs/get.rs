use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobTitle {
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

// Spans, like logs, have an as&sociated level
// `info_span` creates a span at the level info
#[tracing::instrument(name = "Getting all jobs", skip(pool))]
pub async fn get_jobs(pool: web::Data<PgPool>) -> HttpResponse {
    let jobs = sqlx::query!("SELECT * FROM jobs;")
        .fetch_all(pool.get_ref())
        .await;

    match jobs {
        Ok(jobs) => {
            let jobs: Vec<JobTitle> = jobs
                .into_iter()
                .map(|job| JobTitle {
                    title: job.title,
                    company: job.company,
                    workplace_type: job.workplace_type,
                    address: job.address,
                    employment_type: job.employment_type,
                    description: job.description,
                    skills: job.skills,
                    tools: job.tools,
                    min_salary: job.min_salary,
                    max_salary: job.max_salary,
                    salary_rate: job.salary_rate,
                    bonuses: job.bonuses,
                    benefits: job.benefits,
                    is_active: job.is_active,
                })
                .collect();
            println!("JOBS: {:?}", jobs);
            HttpResponse::Ok().json(jobs)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
