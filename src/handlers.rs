use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::models::User;

pub async fn signup(State(pool): State<SqlitePool>, Json(user): Json<User>) -> Json<User> {
    sqlx::query("INSERT OR REPLACE INTO users (display_name, wallet) VALUES (?, ?)")
        .bind(&user.display_name)
        .bind(&user.wallet)
        .execute(&pool)
        .await
        .unwrap();

    Json(user)
}

pub async fn login(State(pool): State<SqlitePool>, Json(payload): Json<User>) -> Json<Option<User>> {
    let found = sqlx::query_as::<_, User>("SELECT * FROM users WHERE wallet = ?")
        .bind(&payload.wallet)
        .fetch_optional(&pool)
        .await
        .unwrap();

    Json(found)
}

pub async fn list_users(State(pool): State<SqlitePool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(users)
}
