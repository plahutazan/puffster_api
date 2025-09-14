use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::models::User;

pub async fn list_users(State(pool): State<SqlitePool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(users)
}