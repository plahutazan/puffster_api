use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

pub async fn init_db() -> SqlitePool {
    // ensure folder exists
    fs::create_dir_all("./data").unwrap();

        // ensure file exists
    if !Path::new("./data/users.db").exists() {
        fs::File::create("./data/users.db").unwrap();
    }

    // connect to SQLite
    let pool = SqlitePool::connect("sqlite://./data/users.db")
        .await
        .unwrap();

    // create table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            display_name TEXT NOT NULL,
            wallet TEXT PRIMARY KEY
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}
