use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::models::User;

pub async fn me(State(pool): State<SqlitePool>, Json(payload): Json<String>) -> Json<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE wallet = ?")
        .bind(&payload)
        .fetch_optional(&pool)
        .await
        .unwrap();

    Json(user)
}