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


pub async fn list_users(State(pool): State<SqlitePool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(users)
}

pub async fn me(State(pool): State<SqlitePool>, Json(payload): Json<String>) -> Json<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE wallet = ?")
        .bind(&payload)
        .fetch_optional(&pool)
        .await
        .unwrap();

    Json(user)
}
