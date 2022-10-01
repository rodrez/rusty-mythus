use argon2::PasswordHasher;

async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool
    ) -> Result<uuid::Uuid, ()> {

    let hasher = argon2::Argon2::new();
    let row: Option<_> = sqlx::query!(
        r#"
        SELECT id, password_hash, salt
        FROM users
        WHERE email = $1
        "#,
        credentials.email
        )
        .fetch_optional(pool)
        .await
        .context("Failed to perform a query to retrieve the stored credentials.")
        .map_err()?;


