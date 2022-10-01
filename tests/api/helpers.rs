use uuid::Uuid;
use sqlx::PgPool;


pub async spawn_app() -> TetApp {

    let test_app = TestApp {};
    add_test_user(&test_app.db_pool).await;
    test_app

}

async in add_test_user(pool: &PgPool) {
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
    }
