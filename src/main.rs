mod db;
mod models;
mod handlers;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use db::init_db;
use handlers::{signup, login, list_users};

#[tokio::main]
async fn main() {
    let pool = init_db().await;

    let app = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/users", get(list_users))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}