use crate::routes::jobs::Job;
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

// "title=SWE&company=Amazon&workplace_type=Remote&address=T&employment_type=Internship&description=Some description&skills=Problem Solver&tools=Vs Code&min_salary=100k&max_salary=350k&salary_rate=Yearly&bonuses=10k Sign In&benefits=Unlimited PTO&is_active=true&created_at=2020-01-01T00:00:00Z&updated_at=2020-01-01T00:00:00Z&agreement=false&first_name=Fabian&last_name=Rodriguez&phone_number=717&company_size=just me"
pub async fn create_job(form: Form<Job>, pool: web::Data<PgPool>) -> HttpResponse {
    // Unique Identifier to trace issues
    let request_id = Uuid::new_v4();

    tracing::info!(
        "
        request_id {} -> 
        JOB DATA: {}, {}, {}, {}, {}, {}, {:#?}, {:#?}, {}, {}, {}, {:#?}, {:#?}, {}, {}, {}, {}, {}
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

    match sqlx::query!(
        r#"
            INSERT INTO jobs (id, title, company, workplace_type, address, employment_type, description, skills, tools, min_salary, max_salary, salary_rate, bonuses, benefits, agreement, created_at, updated_at, is_active, first_name, last_name, company_size, phone_number)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
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
        form.agreement,
        Utc::now(),
        Utc::now(),
        true,
        form.first_name,
        form.last_name,
        form.company_size,
        form.phone_number,
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} -> New job have been saved.",
                request_id
            );
            HttpResponse::Ok().content_type("application/json").finish()
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

// curl 'http://127.0.0.1:8000/v1/create_job' \
//   -H 'Accept: */*' \
//   -H 'Accept-Language: en-US,en;q=0.9' \
//   -H 'Connection: keep-alive' \
//   -H 'Content-Type: application/x-www-form-urlencoded' \
//   -H 'Origin: http://localhost:3001' \
//   -H 'Referer: http://localhost:3001/' \
//   -H 'Sec-Fetch-Dest: empty' \
//   -H 'Sec-Fetch-Mode: cors' \
//   -H 'Sec-Fetch-Site: cross-site' \
//   -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36' \
//   -H 'sec-ch-ua: "Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105"' \
//   -H 'sec-ch-ua-mobile: ?0' \
//   -H 'sec-ch-ua-platform: "Windows"' \
//   --data-raw '{"title":"SWE Intern","company_name":"Unyth","workplace_type":"Remote","address":"8251 Ranchiview Dr Apt 2096","employment_type":"Full Time", \
//   "description":"<p>The Software Engineer is responsible for development of cutting-edge web and mobile applications, with a focus on best practices, scalability, \
//   and high performance. The Software Engineer will work closely with designers and product managers to bring compelling user interfaces to life, either for customers \
//    or internal tools.</p><p></p><h6>Responsibilities:</h6><ol class=\"list-decimal ml-3\"><li class=\"ml-3\"><p>Work collaboratively across departments defining \
//    requirements, designing features and flows, and developing prototypes</p></li><li class=\"ml-3\"><p>Develop software in a cross-platform environment \
//    (iOS, MacOSX)</p></li><li class=\"ml-3\"><p>Identify issues in the application and provide feedback on potential solutions</p></li><li class=\"ml-3\"><p>Participate \
//    in code reviews with other members of the team to ensure code quality standards are met and improving the code base</p></li><li class=\"ml-3\"><p>Participate \
//     in design reviews to assess usability of the application</p></li></ol>","skills":"Ms Office","tools":"Vs Code","minimum_salary":"120","maximum_salary":"200", \
//     "salary_rate":"Yearly","bonuses":"5K Sign In Bonus","benefits":"401K Match","agreement":"true","firstName":"Fabian","lastName":"Rodriguez","company_size":"Just Me", \
//     "phone_number":"7175992268","company":"Unyth"}' \
//   --compressed
