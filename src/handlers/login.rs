use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::models::User;

pub async fn login(State(pool): State<SqlitePool>, Json(payload): Json<User>) -> Json<Option<String>> {
    // Check if user exists
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE wallet = ?")
        .bind(&payload.wallet)
        .fetch_optional(&pool)
        .await
        .unwrap();

    // If exists, return wallet as token
    if let Some(_) = user {
        Json(Some(payload.wallet))
    } else {
        Json(None)
    }
}