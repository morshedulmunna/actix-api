use crate::models::user::User;
use sqlx::PgPool;
use std::error::Error;

pub async fn save_user(pool: &PgPool, user: User) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        "INSERT INTO users (email, password) VALUES ($1, $2)",
        user.email,
        user.password
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<User, Box<dyn Error>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, Box<dyn Error>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn update_user(
    pool: &PgPool,
    user_id: i32,
    updated_user: User,
) -> Result<User, Box<dyn Error>> {
    sqlx::query!(
        "UPDATE users SET email = $1, password = $2 WHERE id = $3",
        updated_user.email,
        updated_user.password,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(updated_user)
}
