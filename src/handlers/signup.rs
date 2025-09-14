use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::models::User;

pub async fn signup(
    State(pool): State<SqlitePool>,
    Json(user): Json<User>
) -> Json<Result<String, String>> {
    // Check if wallet already exists
    let wallet_exists = sqlx::query("SELECT 1 FROM users WHERE wallet = ?")
        .bind(&user.wallet)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .is_some();

    if wallet_exists {
        return Json(Err("Wallet already exists".to_string()));
    }

    // Check if display name already exists
    let name_exists = sqlx::query("SELECT 1 FROM users WHERE display_name = ?")
        .bind(&user.display_name)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .is_some();

    if name_exists {
        return Json(Err("Display name already exists".to_string()));
    }

    // Insert user if both are unique
    sqlx::query("INSERT INTO users (display_name, wallet) VALUES (?, ?)")
        .bind(&user.display_name)
        .bind(&user.wallet)
        .execute(&pool)
        .await
        .unwrap();

    Json(Ok(user.wallet))  // Return wallet as token
}