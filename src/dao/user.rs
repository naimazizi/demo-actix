use crate::model::user::User;
use sqlx::Row;
use uuid::Uuid;

pub async fn check_existing_user(email: &str, pool: &sqlx::MySqlPool) -> bool {
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)")
        .bind(email)
        .fetch_one(pool)
        .await
        .unwrap()
        .get(0);

    exists
}

pub async fn get_user_by_email(
    email: &str,
    pool: &sqlx::MySqlPool,
) -> Result<Option<User>, sqlx::Error> {
    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(pool)
        .await;

    query_result
}

pub async fn get_user_by_id(
    id: &Uuid,
    pool: &sqlx::MySqlPool,
) -> Result<Option<User>, sqlx::Error> {
    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&id.to_string())
        .fetch_optional(pool)
        .await;

    query_result
}

pub async fn insert_new_user(
    name: &str,
    email: &str,
    hashed_password: &str,
    pool: &sqlx::MySqlPool,
) -> Result<User, sqlx::Error> {
    let _ = sqlx::query("INSERT INTO users (name,email,password) VALUES (?, ?, ?)")
        .bind(name)
        .bind(email)
        .bind(hashed_password)
        .execute(pool)
        .await
        .expect(format!("Error in creating users, email: {}", email).as_str());

    Ok(get_user_by_email(email, pool).await.unwrap().unwrap())
}
