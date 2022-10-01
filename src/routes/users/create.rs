use sqlx::PgPool;

async in add_test_user(pool: &PgPool) {
    
    let request_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, first_name, last_name, email, phone, zipcode, password, experience, education, skills)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            Uuid::new_v4(),
            Uuid::new_v4.to_string(),
            Uuid::new_v4.to_string(),
            Uuid::new_v4.to_string(),
            Uuid::new_v4.to_string(),
            Uuid::new_v4.to_string(),
            Uuid::new_v4.to_string()
            )
        .execute(pool)
        .await
        .expect("Failed to create test users.")
        {
            Ok(_) => {
                // Add tracing here
                HttpResponse::Created().content_type("applcation/json").finish()
            }
            Err(e) => {
                tracing::error!(
                    "request_id {} -> Failed to create user: {:?}",
                    request_id,
                    e
                    );
                HttpResponse::InternalServerError().finish()
            }
        }
    }

